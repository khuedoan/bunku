use serde::Deserialize;

pub struct Dns {
    pub params: Params,
    pub outputs: Outputs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub subdomain: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {
    pub host: String,
}

impl Dns {
    pub fn new(params: Params) -> Self {
        let outputs = Outputs {
            host: params.subdomain.clone().unwrap_or("".to_string()),
        };

        Self { params, outputs }
    }

    pub fn resources(&self) -> Option<Vec<String>> {
        None
    }
}
