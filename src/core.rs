use crate::command::Command;
use crate::summand::Summand;
use core::fmt;
use std::{
    collections::VecDeque,
    os::unix::process::ExitStatusExt,
    process::{ExitStatus, Output},
    time::SystemTime,
};
use tokio::process;

pub struct SummandRunner {
    queue: VecDeque<Summand>,
    run_queue: VecDeque<SummandRunnerResult>,
}

#[derive(Debug)]
pub struct SummandRunnerResult {
    pub output: Option<Output>,
    pub error: Option<std::io::Error>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
}

impl fmt::Display for SummandRunnerResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Summand Result(result: {}, run_time: {}ms)",
            self.output.as_ref().unwrap().status,
            self.end_time
                .unwrap()
                .duration_since(self.start_time.unwrap())
                .unwrap_or_else(|_| std::time::Duration::from_millis(0))
                .as_millis(),
        )
    }
}

impl SummandRunnerResult {
    pub fn cli_output_string(self) -> String {
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

    async fn run_command(&self, command: Command) -> Result<SummandRunnerResult, std::io::Error> {
        // Build the command arguments
        let formatted_args: Vec<String> = command
            .arguments
            .iter()
            .map(|a| match &a.name {
                Some(name) => format!("{}={}", name, a.value.clone().unwrap_or_default()),
                None => format!("{}", a.value.clone().unwrap_or_default()),
            })
            .collect();

        println!("args: {:?}, {:?}", formatted_args, command.arguments);
        let start_time = SystemTime::now();
        let result = process::Command::new(command.program)
            .args(formatted_args)
            .output()
            .await;
        let end_time = SystemTime::now();

        match result {
            Ok(output) => Ok(SummandRunnerResult {
                output: Some(output.clone()),
                error: None,
                start_time: Option::from(start_time),
                end_time: Option::from(end_time),
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn execute_next(&mut self) {
        if let Some(summand) = self.queue.pop_front() {
            println!("Executing summand {}", summand.name);
            for command in summand.commands {
                let result = self.run_command(command).await;
                match result {
                    Ok(summand_result) => {
                        let status = match &summand_result.output {
                            Some(out) => out.status,
                            None => ExitStatus::from_raw(0),
                        };
                        println!(
                            "Summand `{}` executed successfully with status {}.",
                            summand.name, status
                        );
                        self.run_queue.push_front(summand_result);
                    }
                    Err(e) => {
                        println!("Error executing command {:?}", e);
                        self.run_queue.push_front(SummandRunnerResult {
                            output: None,
                            error: Some(e),
                            start_time: None,
                            end_time: None,
                        });
                    }
                }
            }
        } else {
            println!("No commands in the queue.");
        }
    }
}
