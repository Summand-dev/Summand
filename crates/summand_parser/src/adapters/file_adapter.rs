use summand_app::summand::Summand;

pub trait FileAdapter {
    fn parse(&self, contents: &str) -> Summand;
}
