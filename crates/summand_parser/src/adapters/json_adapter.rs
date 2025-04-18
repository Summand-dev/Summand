use summand_app::{argument::CommandArgument, command::Command, summand::Summand};

use crate::config::SummandConfig;

use super::file_adapter::FileAdapter;

pub struct JsonAdapter {}

impl FileAdapter for JsonAdapter {
    fn parse(&self, contents: &str) -> Summand {
        let json: SummandConfig =
            serde_json::from_str(contents).expect("JSON was not well-formatted");

        let mut commands = vec![];
        for command in json.commands {
            commands.push(Command::new(
                &command.name,
                Option::from(command.description.unwrap().as_str()),
                &command.program,
                Option::from(
                    command
                        .args
                        .iter()
                        .map(|arg| {
                            return CommandArgument::new(
                                Option::from(arg.name.as_str()),
                                Option::from(
                                    arg.value.clone().or(Some("".to_string())).unwrap().as_str(),
                                ),
                            );
                        })
                        .collect::<Vec<_>>(),
                ),
            ));
        }
        return Summand::new(
            &json.name,
            Option::from(json.description.unwrap().as_str()),
            Option::from(commands),
        );
    }
}
