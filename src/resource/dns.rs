use crate::metadata::Metadata;
use crate::resource::Manifest;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dns {
    pub subdomain: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {
    pub host: String,
}

impl Dns {
    pub fn outputs(&self) -> Outputs {
        match &self.subdomain {
            Some(subdomain) => Outputs {
                host: format!("{subdomain}.localhost"),
            },
            None => Outputs {
                host: "todogeneratedomainhere.localhost".to_string(),
            },
        }
    }

    pub fn provision(&self, _metadata: Metadata, _id: String) -> Vec<Manifest> {
        // DNS will be handled by the gateway and ExternalDNS controller, so no manifests are
        // needed
        vec![]
    }
}
