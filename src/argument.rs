#[derive(Clone, Debug)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl Argument {
    pub fn new(name: Option<&str>, value: Option<&str>) -> Self {
        Self {
            name: match name {
                Some(n) => Some(n.to_string()),
                None => None,
            },
            value: match value {
                Some(v) => Some(v.to_string()),
                None => None,
            },
        }
    }
}
