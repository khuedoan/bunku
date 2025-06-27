use super::{ResourceGenerator, common};
use crate::values::Values;
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

pub struct ConfigMapGenerator;

impl ResourceGenerator for ConfigMapGenerator {
    type Output = ConfigMap;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let configmap_config = values.config_maps.get(name)?;

        if !configmap_config.enabled {
            return None;
        }

        let configmap_name = common::generate_name(values, name);

        let mut labels = common::generate_labels(values, name, "configmap");
        labels.extend(
            configmap_config
                .labels
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let mut annotations = common::generate_annotations(values);
        annotations.extend(
            configmap_config
                .annotations
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let data: BTreeMap<String, String> = configmap_config
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let configmap = ConfigMap {
            metadata: ObjectMeta {
                name: Some(configmap_name),
                labels: Some(labels),
                annotations: if annotations.is_empty() {
                    None
                } else {
                    Some(annotations)
                },
                ..Default::default()
            },
            data: if data.is_empty() { None } else { Some(data) },
            ..Default::default()
        };

        Some(configmap)
    }
}
