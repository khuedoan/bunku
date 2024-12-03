use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub ports: HashMap<String, Port>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Port {
    pub port: i32,
    pub protocol: Option<String>,
}
