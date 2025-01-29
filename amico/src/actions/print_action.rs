use amico_core::entity::Action;
use std::any::Any;

pub struct PrintAction {
    message: String,
}

impl PrintAction {
    pub fn new(message: String) -> Self {
        PrintAction { message }
    }
}

impl Action for PrintAction {
    fn execute(&self) -> Box<dyn Any> {
        println!("{}", self.message);
        Box::new(self.message.clone())
    }
}