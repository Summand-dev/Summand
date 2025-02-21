use summand::{
    argument::CommandArgument, command::Command, core::SummandRunner, summand::Summand,
    variable::SummandVariable,
};

pub fn create_runner() -> SummandRunner {
    let summand_runner = SummandRunner::new();
    return summand_runner;
}

pub fn create_command() -> Command {
    let ls_command = Command::new(
        "LS",
        Some("LS command with variable"),
        "ls",
        Some(vec![
            CommandArgument::new(Some("-A"), None),
            CommandArgument::new(None, Some("$$DIR")),
        ]),
    );
    return ls_command;
}

pub fn create_variable() -> SummandVariable {
    let dir_variable = SummandVariable::new(
        "DIR",
        Option::from("directory to run command in"),
        Option::from(""),
        Option::from(false),
        Option::from(true),
    );
    return dir_variable;
}

pub fn create_summand() -> Summand {
    let ls_command = create_command();
    let dir_variable = create_variable();
    let mut ls_summand = Summand::new("LS", Some("List directory"), Some(vec![ls_command]));
    ls_summand.variables.push(dir_variable);
    return ls_summand;
}

#[tokio::test]
async fn test_summand() {
    let mut runner = create_runner();
    let summand = create_summand();
    runner.add_summand(&summand);

    // Set variable value by user
    runner
        .workspace
        .user_variables
        .insert("DIR".to_string(), "tests/test_dir".to_string());

    runner.execute_next().await;
    let result = runner.pop_output();
    let response = result.unwrap();
    println!("{}", response);
    assert_eq!(
        response.cli_output_string().replace("\n", ""),
        ".gitignore".to_string()
    );
}
