use summand::command;

#[test]
fn create_command() {
    let ls = command::Command::new(
        String::from("LS"),
        Option::from(String::from("List directory")),
        String::from("ls"),
        Option::None,
    );
    assert_eq!(ls.name, "LS")
}
