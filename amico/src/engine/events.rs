use evenio::prelude::*;

#[derive(GlobalEvent, Debug)]
pub struct UserContent(pub String);

#[derive(GlobalEvent, Debug)]
pub struct AgentContent(pub String);

#[derive(GlobalEvent, Debug)]
pub struct UserInput(pub String);

#[derive(GlobalEvent, Debug)]
pub struct RecordStart;

#[derive(GlobalEvent, Debug)]
pub struct RecordFinish;

#[derive(GlobalEvent, Debug)]
pub struct PlaybackFinish;
