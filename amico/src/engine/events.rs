use evenio::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleInput(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct A2aMessageReceived(pub String);

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
