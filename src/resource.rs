pub mod dns;
pub mod route;

use crate::metadata::Metadata;
use gateway_api::apis::standard::httproutes::HTTPRoute;
use k8s_openapi::api::{apps::v1::Deployment, core::v1::Service as KubeService};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "params")]
pub enum Resource {
    Service(KubeService),
    Deployment(Deployment),
    Route(route::Route),
    Dns(dns::Dns),
}

// TODO maybe use generic?
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "params")]
pub enum Manifest {
    HTTPRoute(HTTPRoute),
    Service(KubeService),
    Deployment(Deployment),
}

impl Resource {
    pub fn provision(&self, metadata: Metadata, id: String) -> Vec<Manifest> {
        match self {
            Resource::Deployment(deployment) => vec![Manifest::Deployment(deployment.clone())],
            Resource::Service(service) => vec![Manifest::Service(service.clone())],
            Resource::Route(route) => route.provision(metadata, id),
            Resource::Dns(dns) => dns.provision(metadata, id),
        }
    }
}

#[derive(Debug)]
pub enum ResourceOutputs {
    Dns(dns::Outputs),
    Route(route::Outputs),
}
