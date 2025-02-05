use amico_core::errors::ActionError;
use amico_core::traits::Action;
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
    fn execute(&self) -> Result<(), ActionError> {
        println!("{}", self.message);
        // Simulate some acting time
        thread::sleep(std::time::Duration::from_millis(100));
        Err(ActionError::ExecutingActionError(
            "Print action failed".to_string(),
        ))
    }
}
