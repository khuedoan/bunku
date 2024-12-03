pub mod api_version;
pub mod container;
pub mod metadata;

use api_version::ApiVersion;
use container::Container;
use metadata::Metadata;
use std::collections::HashMap;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container as KubeContainer, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    pub api_version: ApiVersion,
    pub metadata: Metadata,
    pub containers: HashMap<String, Container>,
}

impl From<Deploy> for Deployment {
    fn from(deploy: Deploy) -> Self {
        Deployment {
            metadata: ObjectMeta {
                name: Some(deploy.metadata.name),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: deploy
                            .containers
                            .into_iter()
                            .map(|(name, container)| KubeContainer {
                                name,
                                image: Some(container.image),
                                command: Some(container.command),
                                args: Some(container.args),
                                ..Default::default()
                            })
                            .collect(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
