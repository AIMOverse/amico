pub trait CompletionModel: rig::completion::CompletionModel {}

impl<T> CompletionModel for T where T: rig::completion::CompletionModel {}

pub type Agent<C> = rig::agent::Agent<C>;
