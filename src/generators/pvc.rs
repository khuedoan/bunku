use super::{common, ResourceGenerator};
use crate::values::Values;
use k8s_openapi::api::core::v1::{
    PersistentVolumeClaim, PersistentVolumeClaimSpec, VolumeResourceRequirements,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

pub struct PvcGenerator;

impl ResourceGenerator for PvcGenerator {
    type Output = PersistentVolumeClaim;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let persistence_config = values.persistence.get(name)?;

        if !persistence_config.enabled || persistence_config.r#type != "pvc" {
            return None;
        }

        let pvc_name = common::generate_name(values, name);

        let mut labels = common::generate_labels(values, name, "pvc");
        labels.extend(
            persistence_config
                .labels
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let mut annotations = common::generate_annotations(values);
        annotations.extend(
            persistence_config
                .annotations
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let mut resources = VolumeResourceRequirements::default();
        if let Some(size) = &persistence_config.size {
            let mut requests = BTreeMap::new();
            requests.insert("storage".to_string(), Quantity(size.clone()));
            resources.requests = Some(requests);
        }

        let pvc = PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some(pvc_name),
                labels: Some(labels),
                annotations: if annotations.is_empty() {
                    None
                } else {
                    Some(annotations)
                },
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                access_modes: if persistence_config.access_modes.is_empty() {
                    Some(vec!["ReadWriteOnce".to_string()])
                } else {
                    Some(persistence_config.access_modes.clone())
                },
                resources: Some(resources),
                storage_class_name: persistence_config.storage_class.clone(),
                data_source: persistence_config
                    .data_source
                    .as_ref()
                    .and_then(|ds| serde_json::from_value(ds.clone()).ok()),
                data_source_ref: persistence_config
                    .data_source_ref
                    .as_ref()
                    .and_then(|dsr| serde_json::from_value(dsr.clone()).ok()),
                ..Default::default()
            }),
            ..Default::default()
        };

        Some(pvc)
    }
}
