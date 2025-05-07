use evenio::prelude::*;

#[derive(GlobalEvent, Debug)]
pub struct UserContent(pub String);
