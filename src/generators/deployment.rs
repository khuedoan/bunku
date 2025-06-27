use super::{ResourceGenerator, common};
use crate::values::{Container as ValueContainer, Values};
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec, RollingUpdateDeployment};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, EnvVar, PodSpec, PodTemplateSpec, ResourceRequirements,
    VolumeMount as K8sVolumeMount,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use std::collections::BTreeMap;

pub struct DeploymentGenerator;

impl ResourceGenerator for DeploymentGenerator {
    type Output = Deployment;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let controller = values.controllers.get(name)?;

        if !controller.enabled || controller.r#type != "deployment" {
            return None;
        }

        let deployment_name = common::generate_name(values, name);
        let selector_labels = common::generate_selector_labels(values, name);

        // Merge pod options
        let pod_options = common::merge_pod_options(
            &values.default_pod_options,
            &controller.pod_options,
            &values.default_pod_options_strategy,
        );

        let mut pod_labels = selector_labels.clone();
        pod_labels.extend(
            pod_options
                .labels
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
        if values.global.propagate_global_metadata_to_pods {
            pod_labels.extend(
                values
                    .global
                    .labels
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );
        }

        let mut pod_annotations: BTreeMap<String, String> = pod_options
            .annotations
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if values.global.propagate_global_metadata_to_pods {
            pod_annotations.extend(
                values
                    .global
                    .annotations
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );
        }

        // Convert containers
        let containers: Vec<Container> = controller
            .containers
            .iter()
            .map(|(container_name, container_config)| {
                convert_container(container_name, container_config)
            })
            .collect();

        if containers.is_empty() {
            return None;
        }

        // Convert init containers
        let init_containers: Vec<Container> = controller
            .init_containers
            .iter()
            .map(|(container_name, container_config)| {
                convert_container(container_name, container_config)
            })
            .collect();

        let node_selector: BTreeMap<String, String> = pod_options
            .node_selector
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some(deployment_name),
                labels: Some(common::generate_labels(values, name, "controller")),
                annotations: if values.global.annotations.is_empty() {
                    None
                } else {
                    Some(common::generate_annotations(values))
                },
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: controller.replicas,
                revision_history_limit: Some(controller.revision_history_limit),
                selector: LabelSelector {
                    match_labels: Some(selector_labels),
                    ..Default::default()
                },
                strategy: controller.strategy.as_ref().map(|strategy| {
                    k8s_openapi::api::apps::v1::DeploymentStrategy {
                        type_: Some(strategy.clone()),
                        rolling_update: controller.rolling_update.as_ref().map(|ru| {
                            RollingUpdateDeployment {
                                max_surge: ru
                                    .surge
                                    .as_ref()
                                    .map(|s| IntOrString::String(s.clone())),
                                max_unavailable: ru
                                    .unavailable
                                    .as_ref()
                                    .map(|u| IntOrString::String(u.clone())),
                            }
                        }),
                    }
                }),
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(pod_labels),
                        annotations: if pod_annotations.is_empty() {
                            None
                        } else {
                            Some(pod_annotations)
                        },
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers,
                        init_containers: if init_containers.is_empty() {
                            None
                        } else {
                            Some(init_containers)
                        },
                        service_account_name: controller
                            .service_account
                            .as_ref()
                            .and_then(|sa| sa.name.clone().or_else(|| sa.identifier.clone())),
                        automount_service_account_token: Some(
                            pod_options.automount_service_account_token,
                        ),
                        dns_policy: pod_options.dns_policy,
                        enable_service_links: Some(pod_options.enable_service_links),
                        hostname: pod_options.hostname,
                        host_ipc: Some(pod_options.host_ipc),
                        host_network: Some(pod_options.host_network),
                        host_pid: Some(pod_options.host_pid),
                        node_selector: if node_selector.is_empty() {
                            None
                        } else {
                            Some(node_selector)
                        },
                        priority_class_name: pod_options.priority_class_name,
                        restart_policy: pod_options.restart_policy,
                        runtime_class_name: pod_options.runtime_class_name,
                        scheduler_name: pod_options.scheduler_name,
                        termination_grace_period_seconds: pod_options
                            .termination_grace_period_seconds,
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        Some(deployment)
    }
}

fn convert_container(name: &str, config: &ValueContainer) -> Container {
    Container {
        name: name.to_string(),
        image: Some(config.image.clone()),
        command: config.command.clone(),
        args: config.args.clone(),
        env: if config.env.is_empty() {
            None
        } else {
            Some(
                config
                    .env
                    .iter()
                    .map(|(key, env_var)| EnvVar {
                        name: key.clone(),
                        value: env_var.value.clone(),
                        value_from: env_var
                            .value_from
                            .as_ref()
                            .and_then(|vf| serde_json::from_value(vf.clone()).ok()),
                    })
                    .collect(),
            )
        },
        ports: if config.ports.is_empty() {
            None
        } else {
            Some(
                config
                    .ports
                    .iter()
                    .map(|(port_name, port_config)| ContainerPort {
                        name: Some(port_name.clone()),
                        container_port: port_config.container_port,
                        protocol: port_config.protocol.clone(),
                        ..Default::default()
                    })
                    .collect(),
            )
        },
        resources: config.resources.as_ref().map(|res| ResourceRequirements {
            limits: res.limits.as_ref().map(|limits| {
                limits
                    .iter()
                    .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                    .collect()
            }),
            requests: res.requests.as_ref().map(|requests| {
                requests
                    .iter()
                    .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                    .collect()
            }),
            ..Default::default()
        }),
        security_context: config
            .security_context
            .as_ref()
            .and_then(|sc| serde_json::from_value(sc.clone()).ok()),
        volume_mounts: if config.volume_mounts.is_empty() {
            None
        } else {
            Some(
                config
                    .volume_mounts
                    .iter()
                    .map(|vm| K8sVolumeMount {
                        name: vm.name.clone(),
                        mount_path: vm.mount_path.clone(),
                        read_only: vm.read_only,
                        sub_path: vm.sub_path.clone(),
                        ..Default::default()
                    })
                    .collect(),
            )
        },
        liveness_probe: config
            .liveness_probe
            .as_ref()
            .and_then(|probe| serde_json::from_value(probe.clone()).ok()),
        readiness_probe: config
            .readiness_probe
            .as_ref()
            .and_then(|probe| serde_json::from_value(probe.clone()).ok()),
        startup_probe: config
            .startup_probe
            .as_ref()
            .and_then(|probe| serde_json::from_value(probe.clone()).ok()),
        ..Default::default()
    }
}
