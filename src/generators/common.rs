use crate::values::{PodOptions, Values};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use std::collections::BTreeMap;

pub fn generate_name(_values: &Values, resource_name: &str) -> String {
    resource_name.to_string()
}

pub fn generate_labels(
    values: &Values,
    resource_name: &str,
    resource_type: &str,
) -> BTreeMap<String, String> {
    let mut labels = BTreeMap::new();

    // Standard labels
    labels.insert(
        "app.kubernetes.io/name".to_string(),
        generate_name(values, resource_name),
    );
    labels.insert(
        "app.kubernetes.io/instance".to_string(),
        "release".to_string(),
    );

    // Add resource-specific labels
    match resource_type {
        "controller" => {
            labels.insert(
                "app.kubernetes.io/controller".to_string(),
                resource_name.to_string(),
            );
        }
        "service" => {
            labels.insert(
                "app.kubernetes.io/service".to_string(),
                resource_name.to_string(),
            );
        }
        _ => {}
    }

    // Add global labels
    for (key, value) in &values.global.labels {
        labels.insert(key.clone(), value.clone());
    }

    labels
}

pub fn generate_annotations(values: &Values) -> BTreeMap<String, String> {
    values
        .global
        .annotations
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn generate_metadata(values: &Values, resource_name: &str, resource_type: &str) -> ObjectMeta {
    ObjectMeta {
        name: Some(generate_name(values, resource_name)),
        labels: Some(generate_labels(values, resource_name, resource_type)),
        annotations: if values.global.annotations.is_empty() {
            None
        } else {
            Some(generate_annotations(values))
        },
        ..Default::default()
    }
}

pub fn generate_selector_labels(values: &Values, resource_name: &str) -> BTreeMap<String, String> {
    let mut labels = BTreeMap::new();
    labels.insert(
        "app.kubernetes.io/name".to_string(),
        generate_name(values, resource_name),
    );
    labels.insert(
        "app.kubernetes.io/instance".to_string(),
        "release".to_string(),
    );
    labels.insert(
        "app.kubernetes.io/controller".to_string(),
        resource_name.to_string(),
    );
    labels
}

pub fn merge_pod_options(
    default_options: &PodOptions,
    controller_options: &PodOptions,
    strategy: &str,
) -> PodOptions {
    match strategy {
        "merge" => {
            let mut merged = default_options.clone();

            // Merge annotations
            merged
                .annotations
                .extend(controller_options.annotations.clone());

            // Merge labels
            merged.labels.extend(controller_options.labels.clone());

            // Override other fields if they are set in controller options
            if controller_options.automount_service_account_token {
                merged.automount_service_account_token =
                    controller_options.automount_service_account_token;
            }
            if controller_options.dns_policy.is_some() {
                merged.dns_policy = controller_options.dns_policy.clone();
            }
            if !controller_options.enable_service_links {
                merged.enable_service_links = controller_options.enable_service_links;
            }
            if controller_options.hostname.is_some() {
                merged.hostname = controller_options.hostname.clone();
            }
            if controller_options.host_ipc {
                merged.host_ipc = controller_options.host_ipc;
            }
            if controller_options.host_network {
                merged.host_network = controller_options.host_network;
            }
            if controller_options.host_pid {
                merged.host_pid = controller_options.host_pid;
            }

            merged
        }
        "overwrite" => {
            // Use controller options if they have non-default values, otherwise use defaults
            let mut result = controller_options.clone();

            // Only use defaults if controller options are empty/default
            if result.annotations.is_empty() {
                result.annotations = default_options.annotations.clone();
            }
            if result.labels.is_empty() {
                result.labels = default_options.labels.clone();
            }
            if result.node_selector.is_empty() {
                result.node_selector = default_options.node_selector.clone();
            }
            if result.tolerations.is_empty() {
                result.tolerations = default_options.tolerations.clone();
            }
            if result.topology_spread_constraints.is_empty() {
                result.topology_spread_constraints =
                    default_options.topology_spread_constraints.clone();
            }

            result
        }
        _ => {
            // Default case - same as overwrite for any unknown strategy
            let mut result = controller_options.clone();

            // Only use defaults if controller options are empty/default
            if result.annotations.is_empty() {
                result.annotations = default_options.annotations.clone();
            }
            if result.labels.is_empty() {
                result.labels = default_options.labels.clone();
            }
            if result.node_selector.is_empty() {
                result.node_selector = default_options.node_selector.clone();
            }
            if result.tolerations.is_empty() {
                result.tolerations = default_options.tolerations.clone();
            }
            if result.topology_spread_constraints.is_empty() {
                result.topology_spread_constraints =
                    default_options.topology_spread_constraints.clone();
            }

            result
        }
    }
}
