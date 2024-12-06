use crate::metadata::Metadata;
use crate::resource::Manifest;

use gateway_api::apis::standard::httproutes::HTTPRoute;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub host: String,
    pub path: String,
    pub port: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {}

impl Route {
    pub fn outputs(&self) -> Outputs {
        Outputs {}
    }

    pub fn provision(&self, metadata: Metadata, id: String) -> Vec<Manifest> {
        vec![Manifest::HTTPRoute(HTTPRoute {
            metadata: ObjectMeta {
                name: Some(format!("{}-{}", metadata.name, id)),
                ..Default::default()
            },
            ..Default::default()
        })]
    }
}
