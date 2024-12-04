use serde::Deserialize;

pub struct Dns {
    pub params: Params,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub subdomain: Option<String>,
}
