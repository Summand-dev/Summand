use super::application::Application;

pub trait ServiceProvider {
    fn boot(&self, app: &mut Application);
}
