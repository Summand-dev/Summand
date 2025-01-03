#[derive(Debug)]
pub struct Argument {
    pub name: String,
    pub value: Option<String>,
}

impl Argument {
    pub fn new(name: &str, value: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            value: Option::from(value.unwrap_or("").to_string()),
        }
    }
}
