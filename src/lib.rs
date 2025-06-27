pub mod error;
pub mod generators;
pub mod values;

pub use generators::generate_all_resources;
pub use values::Values;

use serde_json::Value;
use std::fs;

pub fn process_values_file(filename: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let values: Values = toml::from_str(&content)?;
    let resources = generate_all_resources(&values);
    Ok(resources)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_simple_deployment() {
        let toml_content = r#"
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 2

[controllers.main.containers.app]
image = "nginx:latest"

[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"
"#;

        let values: Values = toml::from_str(toml_content).unwrap();
        let resources = generate_all_resources(&values);

        assert!(!resources.is_empty());
        // Should generate a deployment
        assert_eq!(resources.len(), 1);
    }

    #[test]
    fn test_deployment_with_service() {
        let toml_content = r#"
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "nginx:latest"

[controllers.main.containers.app.ports.http]
containerPort = 80

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
"#;

        let values: Values = toml::from_str(toml_content).unwrap();
        let resources = generate_all_resources(&values);

        // Should generate a deployment and a service
        assert_eq!(resources.len(), 2);
    }

    #[test]
    fn test_process_values_file() {
        let toml_content = r#"
[global]

[controllers.main]
enabled = true
type = "deployment"

[controllers.main.containers.app]
image = "busybox:latest"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();

        let resources = process_values_file(temp_file.path().to_str().unwrap()).unwrap();
        assert!(!resources.is_empty());
    }

    #[test]
    fn test_configmap_generation() {
        let toml_content = r#"
[global]

[configMaps.config]
enabled = true

[configMaps.config.data]
key1 = "value1"
key2 = "value2"
"#;

        let values: Values = toml::from_str(toml_content).unwrap();
        let resources = generate_all_resources(&values);

        assert_eq!(resources.len(), 1);
    }

    #[test]
    fn test_pvc_generation() {
        let toml_content = r#"
[global]

[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]
"#;

        let values: Values = toml::from_str(toml_content).unwrap();
        let resources = generate_all_resources(&values);

        assert_eq!(resources.len(), 1);
    }

    #[test]
    fn test_list_output_format() {
        let toml_content = r#"
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "nginx:latest"

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
"#;

        let values: Values = toml::from_str(toml_content).unwrap();
        let resources = generate_all_resources(&values);

        // Create List object
        let list = serde_json::json!({
            "apiVersion": "v1",
            "kind": "List",
            "items": resources
        });

        // Verify List structure
        assert_eq!(list["apiVersion"], "v1");
        assert_eq!(list["kind"], "List");
        assert!(list["items"].is_array());
        assert_eq!(list["items"].as_array().unwrap().len(), 2);
    }
}
