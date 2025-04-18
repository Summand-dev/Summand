use std::fs;

use summand_parser::parser::SummandParser;

#[test]
fn parse_json() {
    let parser = SummandParser::new();

    print!("hello");

    let contents = fs::read_to_string("./tests/files/bat_install.json").expect("File not found");

    print!("read file contents: {}", contents);

    let summand = parser.parse(&contents);

    println!("{:?}", summand);
    assert_eq!(summand.name, "imported")
}
