use crate::command::Command;
use crate::summand::Summand;
use std::{collections::VecDeque, process::Output};
use tokio::process;

pub struct SummandRunner {
    queue: VecDeque<Summand>,
    run_queue: VecDeque<SummandRunnerResult>,
}

#[derive(Debug)]
pub struct SummandRunnerResult {
    pub output: Option<Output>,
    pub error: Option<std::io::Error>,
}

impl SummandRunnerResult {
    pub fn to_string(self) -> String {
        match self.output {
            Some(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            None => "".to_string(),
        }
    }
}

impl SummandRunner {
    pub fn new() -> Self {
        SummandRunner {
            queue: VecDeque::new(),
            run_queue: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn pop_output(&mut self) -> Result<SummandRunnerResult, String> {
        if let Some(result) = self.run_queue.pop_front() {
            Ok(result)
        } else {
            Err("No result in queue".to_string())
        }
    }

    pub fn add_summand(&mut self, summand: &Summand) {
        self.queue.push_back(summand.to_owned());
    }

    async fn run_command(&self, command: Command) -> Result<Output, std::io::Error> {
        // Build the command arguments
        let formatted_args: Vec<String> = command
            .arguments
            .iter()
            .map(|a| match &a.name {
                Some(name) => format!("--{}={}", name, a.value.clone().unwrap_or_default()),
                None => format!("{}", a.value.clone().unwrap_or_default()),
            })
            .collect();

        println!("args: {:?}, {:?}", formatted_args, command.arguments);
        let result = process::Command::new(command.program)
            .args(formatted_args)
            .output()
            .await;

        match result {
            Ok(output) => Ok(output),
            Err(err) => Err(err),
        }
    }

    pub async fn execute_next(&mut self) {
        if let Some(summand) = self.queue.pop_front() {
            println!("Executing summand {}", summand.name);
            for command in summand.commands {
                let result = self.run_command(command).await;
                match result {
                    Ok(output) => {
                        println!(
                            "Summand `{}` executed successfully with status {}.",
                            summand.name, output.status
                        );
                        self.run_queue.push_front(SummandRunnerResult {
                            output: Some(output.clone()),
                            error: None,
                        });
                    }
                    Err(e) => {
                        println!("Error executing command {:?}", e);
                        self.run_queue.push_front(SummandRunnerResult {
                            output: None,
                            error: Some(e),
                        });
                    }
                }
            }
        } else {
            println!("No commands in the queue.");
        }
    }
}
