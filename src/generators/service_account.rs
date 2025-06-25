use super::{common, ResourceGenerator};
use crate::values::Values;
use k8s_openapi::api::core::v1::{ObjectReference, ServiceAccount};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub struct ServiceAccountGenerator;

impl ResourceGenerator for ServiceAccountGenerator {
    type Output = ServiceAccount;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let sa_config = values.service_account.get(name)?;

        if !sa_config.enabled {
            return None;
        }

        let sa_name = common::generate_name(values, name);

        let mut labels = common::generate_labels(values, name, "serviceaccount");
        labels.extend(sa_config.labels.iter().map(|(k, v)| (k.clone(), v.clone())));

        let mut annotations = common::generate_annotations(values);
        annotations.extend(
            sa_config
                .annotations
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let secrets = if sa_config.secrets.is_empty() {
            None
        } else {
            Some(
                sa_config
                    .secrets
                    .iter()
                    .map(|secret_name| ObjectReference {
                        name: Some(secret_name.clone()),
                        ..Default::default()
                    })
                    .collect(),
            )
        };

        let service_account = ServiceAccount {
            metadata: ObjectMeta {
                name: Some(sa_name),
                labels: Some(labels),
                annotations: if annotations.is_empty() {
                    None
                } else {
                    Some(annotations)
                },
                ..Default::default()
            },
            automount_service_account_token: sa_config.automount_service_account_token,
            secrets,
            ..Default::default()
        };

        Some(service_account)
    }
}
