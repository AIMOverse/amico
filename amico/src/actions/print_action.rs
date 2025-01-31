use amico_core::traits::Action;
use std::any::Any;
use std::thread;

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
        // Simulate some acting time
        thread::sleep(std::time::Duration::from_millis(100));
        Box::new(self.message.clone())
    }
}
