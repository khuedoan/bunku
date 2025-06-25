pub mod configmap;
pub mod deployment;
pub mod pvc;
pub mod service;
pub mod service_account;
// pub mod http_route;
pub mod common;

use crate::values::Values;
use serde_json::Value;

pub trait ResourceGenerator {
    type Output;

    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output>;
}

pub fn generate_all_resources(values: &Values) -> Vec<Value> {
    let mut resources = Vec::new();

    // Generate controllers (only deployments for now)
    for (name, controller) in &values.controllers {
        if controller.enabled && controller.r#type == "deployment" {
            if let Some(deployment) = deployment::DeploymentGenerator.generate(values, name) {
                resources.push(serde_json::to_value(deployment).unwrap());
            }
        }
    }

    // Generate services
    for (name, service) in &values.service {
        if service.enabled {
            if let Some(svc) = service::ServiceGenerator.generate(values, name) {
                resources.push(serde_json::to_value(svc).unwrap());
            }
        }
    }

    // Generate ConfigMaps
    for (name, configmap) in &values.config_maps {
        if configmap.enabled {
            if let Some(cm) = configmap::ConfigMapGenerator.generate(values, name) {
                resources.push(serde_json::to_value(cm).unwrap());
            }
        }
    }

    // Generate ServiceAccounts
    for (name, service_account) in &values.service_account {
        if service_account.enabled {
            if let Some(sa) = service_account::ServiceAccountGenerator.generate(values, name) {
                resources.push(serde_json::to_value(sa).unwrap());
            }
        }
    }

    // Generate PVCs
    for (name, persistence) in &values.persistence {
        if persistence.enabled && persistence.r#type == "pvc" {
            if let Some(pvc) = pvc::PvcGenerator.generate(values, name) {
                resources.push(serde_json::to_value(pvc).unwrap());
            }
        }
    }

    // // Generate HTTPRoutes
    // for (name, route) in &values.route {
    //     if route.enabled {
    //         if let Some(http_route) = http_route::HttpRouteGenerator.generate(values, name) {
    //             resources.push(serde_json::to_value(http_route).unwrap());
    //         }
    //     }
    // }

    resources
}
