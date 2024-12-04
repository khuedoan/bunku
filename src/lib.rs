pub mod api_version;
pub mod args;
pub mod container;
pub mod metadata;
pub mod provisioner;
pub mod service;

use api_version::ApiVersion;
use container::Container;
use metadata::Metadata;
use service::Service;
use std::collections::{BTreeMap, HashMap};

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container as KubeContainer, PodSpec, PodTemplateSpec, Service as KubeService,
            ServicePort, ServiceSpec,
        },
    },
    apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload {
    pub api_version: ApiVersion,
    pub metadata: Metadata,
    pub containers: HashMap<String, Container>,
    pub service: Option<Service>,
    pub resources: Option<HashMap<String, Resource>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "params")]
pub enum Resource {
    Route(provisioner::route::Params),
    Dns(provisioner::dns::Params),
}

impl Workload {
    pub fn deployment(self) -> Deployment {
        let labels = BTreeMap::from([(
            "app.kubernetes.io/name".to_string(),
            self.metadata.name.clone(),
        )]);
        Deployment {
            metadata: ObjectMeta {
                name: Some(self.metadata.name.clone()),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                selector: LabelSelector {
                    match_labels: Some(labels.clone()),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(labels),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: self
                            .containers
                            .into_iter()
                            .map(|(name, container)| KubeContainer {
                                name,
                                image: Some(container.image),
                                command: container.command,
                                args: container.args,
                                ..Default::default()
                            })
                            .collect(),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn service(self) -> Option<KubeService> {
        match self.service {
            None => None,
            Some(service) => {
                let labels = BTreeMap::from([(
                    "app.kubernetes.io/name".to_string(),
                    self.metadata.name.clone(),
                )]);
                Some(KubeService {
                    metadata: ObjectMeta {
                        name: Some(self.metadata.name.clone()),
                        ..Default::default()
                    },
                    spec: Some(ServiceSpec {
                        selector: Some(labels.clone()),
                        ports: Some(
                            service
                                .ports
                                .into_iter()
                                .map(|(name, port)| ServicePort {
                                    name: Some(name),
                                    port: port.port,
                                    protocol: port.protocol,
                                    ..Default::default()
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
        }
    }

    pub fn resources(self) -> Vec<String> {
        self.resources
            .unwrap_or_default()
            .into_values()
            .map(|_resource| "".to_string())
            .collect()
    }
}
