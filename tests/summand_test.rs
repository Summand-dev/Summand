use summand::argument::Argument;
use summand::command::Command;
use summand::summand::Summand;

fn create_command() -> Command {
    let ls_command = Command::new(
        "LS",
        Some("List directory"),
        "ls",
        Some(vec![Argument::new("directory", Some("."))]),
    );
    return ls_command;
}

fn create_summand() -> Summand {
    let ls_command = create_command();
    let ls_summand = Summand::new("LS", Some("List directory"), Some(vec![ls_command]));
    return ls_summand;
}

#[test]
fn test_summand() {
    let ls_summand = create_summand();
    println!("{:?}", ls_summand)
}
