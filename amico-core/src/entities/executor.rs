use crate::entities::Action;

/// Executes the given action and returns the response.
pub fn execute_action(action: &dyn Action) -> String {
    action.execute()
}
