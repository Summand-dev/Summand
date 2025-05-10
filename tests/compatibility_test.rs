use summand_app::{
    argument::CommandArgument, command::Command, compatiblity::SummandCompatiblity,
    core::SummandRunner, summand::Summand,
};

pub fn create_runner() -> SummandRunner {
    let summand_runner = SummandRunner::new();
    return summand_runner;
}

pub fn create_command() -> Command {
    let echo_command = Command::new(
        "Echo",
        Some("Echo a fixed value"),
        "echo",
        Some(vec![CommandArgument::new(None, Some("Test"))]),
    );
    return echo_command;
}

pub fn create_summand() -> Summand {
    let echo_command = create_command();
    let mut echo_summand = Summand::new("Echo", Some("Echo test"), Some(vec![echo_command]));
    echo_summand.compatiblities.push(SummandCompatiblity::new(
        summand_app::compatiblity::SummandCompatiblityType::Platform,
        Option::None,
        Option::from(vec!["windows".to_string()]),
    ));
    return echo_summand;
}

#[tokio::test]
async fn test_summand() {
    let echo_summand = create_summand();
    let mut runner = create_runner();
    runner.add_summand(&echo_summand);
    runner.execute_next().await;
    let result = runner.pop_output();
    let response = result.unwrap();
    println!("{}", response);
    assert_eq!(response.cli_output_string(), "Test\n".to_string());
}
