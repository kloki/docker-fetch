use std::collections::HashMap;

use chrono::{DateTime, Local, TimeDelta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub name: String,
    pub id: String,
    pub state: State,
    pub created: DateTime<Local>,
    pub config: Config,
    pub network_settings: NetworkSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub status: String,
    pub started_at: DateTime<Local>,
    pub finished_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub hostname: String,
    pub env: Vec<String>,
    pub image: String,
    pub cmd: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub ports: HashMap<String, Vec<Port>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub host_ip: String,
    pub host_port: String,
}

impl Container {
    pub fn uptime(&self) -> TimeDelta {
        if self.state.status == *"running" {
            Local::now() - self.state.started_at
        } else {
            self.state.finished_at - self.state.started_at
        }
    }
}
