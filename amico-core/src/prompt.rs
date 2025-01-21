pub trait Prompt {
    fn to_prompt(&self) -> String;
}
