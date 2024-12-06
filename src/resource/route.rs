use crate::metadata::Metadata;
use crate::resource::Manifest;

use gateway_api::apis::standard::httproutes::{
    HTTPRoute, HTTPRouteParentRefs, HTTPRouteRules, HTTPRouteRulesBackendRefs,
    HTTPRouteRulesMatches, HTTPRouteRulesMatchesPath, HTTPRouteRulesMatchesPathType, HTTPRouteSpec,
};
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
            spec: HTTPRouteSpec {
                parent_refs: Some(vec![HTTPRouteParentRefs {
                    name: "default".to_string(),
                    namespace: Some("gatewway".to_string()),
                    ..Default::default()
                }]),
                hostnames: Some(vec![self.host.clone()]),
                rules: Some(vec![HTTPRouteRules {
                    matches: Some(vec![HTTPRouteRulesMatches {
                        path: Some(HTTPRouteRulesMatchesPath {
                            r#type: Some(HTTPRouteRulesMatchesPathType::PathPrefix),
                            value: Some(self.path.clone()),
                        }),
                        ..Default::default()
                    }]),
                    backend_refs: Some(vec![HTTPRouteRulesBackendRefs {
                        name: format!("{}-{}", metadata.name, id),
                        port: Some(3000), // TODO read from service
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
            },
            ..Default::default()
        })]
    }
}
