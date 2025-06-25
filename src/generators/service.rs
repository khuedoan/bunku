use super::{common, ResourceGenerator};
use crate::values::Values;
use k8s_openapi::api::core::v1::{Service, ServicePort, ServiceSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub struct ServiceGenerator;

impl ResourceGenerator for ServiceGenerator {
    type Output = Service;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let service_config = values.service.get(name)?;

        if !service_config.enabled {
            return None;
        }

        let service_name = common::generate_name(values, name);

        // Generate selector labels - either from explicit controller or infer from name
        let controller_name = service_config
            .controller
            .clone()
            .unwrap_or_else(|| name.to_string());
        let mut selector_labels = common::generate_selector_labels(values, &controller_name);

        // Add extra selector labels
        selector_labels.extend(
            service_config
                .extra_selector_labels
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let ports: Vec<ServicePort> = service_config
            .ports
            .iter()
            .map(|(port_name, port_config)| ServicePort {
                name: Some(port_name.clone()),
                port: port_config.port,
                target_port: port_config
                    .target_port
                    .map(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int),
                protocol: port_config.protocol.clone(),
                node_port: port_config.node_port,
                app_protocol: port_config.app_protocol.clone(),
            })
            .collect();

        if ports.is_empty() {
            return None;
        }

        let mut service_labels = common::generate_labels(values, name, "service");
        service_labels.extend(
            service_config
                .labels
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let mut service_annotations = common::generate_annotations(values);
        service_annotations.extend(
            service_config
                .annotations
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );

        let service = Service {
            metadata: ObjectMeta {
                name: Some(service_name),
                labels: Some(service_labels),
                annotations: if service_annotations.is_empty() {
                    None
                } else {
                    Some(service_annotations)
                },
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some(service_config.r#type.clone()),
                selector: Some(selector_labels),
                ports: Some(ports),
                cluster_ip: service_config.cluster_ip.clone(),
                load_balancer_ip: service_config.load_balancer_ip.clone(),
                load_balancer_source_ranges: service_config.load_balancer_source_ranges.clone(),
                load_balancer_class: service_config.load_balancer_class.clone(),
                external_name: service_config.external_name.clone(),
                internal_traffic_policy: service_config.internal_traffic_policy.clone(),
                external_traffic_policy: service_config.external_traffic_policy.clone(),
                allocate_load_balancer_node_ports: service_config.allocate_load_balancer_node_ports,
                session_affinity: service_config.session_affinity.clone(),
                session_affinity_config: service_config
                    .session_affinity_config
                    .as_ref()
                    .and_then(|sac| serde_json::from_value(sac.clone()).ok()),
                external_ips: service_config.external_ips.clone(),
                publish_not_ready_addresses: service_config.publish_not_ready_addresses,
                ip_family_policy: service_config.ip_family_policy.clone(),
                ip_families: service_config.ip_families.clone(),
                ..Default::default()
            }),
            ..Default::default()
        };

        Some(service)
    }
}
