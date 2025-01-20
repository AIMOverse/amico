pub trait Prompt {
    fn into_prompt(&self) -> String;
}