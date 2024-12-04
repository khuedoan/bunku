use serde::Deserialize;

pub struct Route {
    pub params: Params,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub host: String,
    pub path: String,
    pub port: String,
}