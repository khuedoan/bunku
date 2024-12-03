use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
}
