use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub name: String,
}
