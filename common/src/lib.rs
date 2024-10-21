use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub command: String, 
    pub timestamp: String,
    pub timestamp1: String,
    pub timestamp2: String,
    pub timestamp3: String,
    pub user: String,
    pub environment: Environment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub shell: String,
    pub os: String,
}

pub mod command_proto {
    include!(concat!(env!("OUT_DIR"), "/command.rs"));
}
