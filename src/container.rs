use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub image: String,
    pub command: Vec<String>,
    pub args: Vec<String>,
}
