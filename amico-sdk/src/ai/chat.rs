pub struct Message {
    pub role: String,
    pub content: String,
}

pub type ChatHistory = Vec<Message>;
