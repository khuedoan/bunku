pub mod api_version;
pub mod container;
pub mod metadata;

use api_version::ApiVersion;
use container::Container;
use metadata::Metadata;
use std::collections::{BTreeMap, HashMap};

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container as KubeContainer, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub api_version: ApiVersion,
    pub metadata: Metadata,
    pub containers: HashMap<String, Container>,
}

impl From<App> for Deployment {
    fn from(app: App) -> Self {
        let labels = BTreeMap::from([(
            "app.kubernetes.io/name".to_string(),
            app.metadata.name.clone(),
        )]);
        Deployment {
            metadata: ObjectMeta {
                name: Some(app.metadata.name.clone()),
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
                        containers: app
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
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
