use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub image: String,
    pub command: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
}
