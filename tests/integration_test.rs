use bunku::process_values_file;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Helper function to get expected filenames for a resource
fn get_resource_filename(resource: &Value) -> String {
    let kind = resource["kind"].as_str().unwrap_or("unknown");
    let name = resource["metadata"]["name"].as_str().unwrap_or("unnamed");

    format!("{}-{}.json", kind, name)
}

/// Helper function to apply name prefix to resources (simulating CLI behavior)
fn apply_name_prefix(mut resources: Vec<Value>, app_name: &str) -> Vec<Value> {
    for resource in &mut resources {
        if let Some(metadata) = resource.get_mut("metadata") {
            if let Some(name) = metadata.get("name").and_then(|n| n.as_str()) {
                let new_name = format!("{}-{}", app_name, name);
                metadata["name"] = serde_json::Value::String(new_name);
            }
        }
    }
    resources
}

/// Helper function to load all expected JSON files from an example's output directory
fn load_expected_resources(example_name: &str) -> HashMap<String, Value> {
    let output_dir = format!("examples/{}/output", example_name);
    let mut expected_resources = HashMap::new();

    let entries = fs::read_dir(&output_dir)
        .unwrap_or_else(|_| panic!("Failed to read output directory: {}", output_dir));

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Failed to read file: {}", path.display()));
            let resource: Value = serde_json::from_str(&content)
                .unwrap_or_else(|_| panic!("Failed to parse JSON from: {}", path.display()));

            expected_resources.insert(filename, resource);
        }
    }

    expected_resources
}

/// Helper function to test an example against its expected output files
fn test_example_output(example_name: &str, app_name: &str) {
    let app_toml_path = format!("examples/{}/app.toml", example_name);

    // Verify the app.toml file exists
    assert!(
        Path::new(&app_toml_path).exists(),
        "Example file does not exist: {}",
        app_toml_path
    );

    // Process the TOML file
    let resources = process_values_file(&app_toml_path)
        .unwrap_or_else(|e| panic!("Failed to process {}: {}", app_toml_path, e));

    // Apply name prefix (simulating CLI behavior)
    let actual_resources = apply_name_prefix(resources, app_name);

    // Load expected output files
    let expected_resources = load_expected_resources(example_name);

    // Convert actual resources to filename -> resource map
    let mut actual_resources_map = HashMap::new();
    for resource in &actual_resources {
        let filename = get_resource_filename(resource);
        actual_resources_map.insert(filename, resource.clone());
    }

    // Compare resource counts
    assert_eq!(
        actual_resources_map.len(),
        expected_resources.len(),
        "Resource count mismatch for {}: expected {}, got {}",
        example_name,
        expected_resources.len(),
        actual_resources_map.len()
    );

    // Compare each resource
    for (filename, expected_resource) in &expected_resources {
        let actual_resource = actual_resources_map
            .get(filename)
            .unwrap_or_else(|| panic!("Missing resource file {} for {}", filename, example_name));

        // Convert to JSON values for comparison
        let actual_json = serde_json::to_value(actual_resource)
            .expect("Failed to serialize actual resource to JSON");

        assert_eq!(
            actual_json,
            *expected_resource,
            "Resource {} mismatch for {}\nActual: {}\nExpected: {}",
            filename,
            example_name,
            serde_json::to_string_pretty(&actual_json).unwrap(),
            serde_json::to_string_pretty(expected_resource).unwrap()
        );
    }
}

#[test]
fn test_hello_world_example() {
    test_example_output("hello-world", "hello-world");
}

#[test]
fn test_nginx_example() {
    test_example_output("nginx", "nginx");
}

#[test]
fn test_podinfo_example() {
    test_example_output("podinfo", "podinfo");
}

#[test]
fn test_full_example() {
    test_example_output("full", "myapp");
}

#[test]
fn test_all_examples_have_expected_outputs() {
    let examples = ["hello-world", "nginx", "podinfo", "full"];

    for example in &examples {
        let app_toml = format!("examples/{}/app.toml", example);
        let output_dir = format!("examples/{}/output", example);

        assert!(
            Path::new(&app_toml).exists(),
            "Missing app.toml for {}",
            example
        );
        assert!(
            Path::new(&output_dir).exists(),
            "Missing output directory for {}",
            example
        );

        // Verify output directory has JSON files
        let entries = fs::read_dir(&output_dir)
            .unwrap_or_else(|_| panic!("Failed to read output directory for {}", example));

        let json_files: Vec<_> = entries
            .filter_map(|e| {
                let entry = e.ok()?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        assert!(
            !json_files.is_empty(),
            "No JSON files found in output directory for {}",
            example
        );

        // Verify each JSON file is valid
        for json_file in json_files {
            let content = fs::read_to_string(&json_file)
                .unwrap_or_else(|_| panic!("Failed to read {}", json_file.display()));
            serde_json::from_str::<Value>(&content)
                .unwrap_or_else(|_| panic!("Invalid JSON in {}", json_file.display()));
        }
    }
}

#[test]
fn test_hello_world_specific_structure() {
    let expected = load_expected_resources("hello-world");
    assert_eq!(expected.len(), 1);

    let deployment = expected
        .get("Deployment-hello-world-main.json")
        .expect("Missing Deployment-hello-world-main.json");

    assert_eq!(deployment["kind"], "Deployment");
    assert_eq!(deployment["metadata"]["name"], "hello-world-main");
    assert_eq!(deployment["spec"]["replicas"], 1);
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["image"],
        "docker.io/library/busybox:1.36"
    );
}

#[test]
fn test_nginx_specific_structure() {
    let expected = load_expected_resources("nginx");
    assert_eq!(expected.len(), 2);

    // Check Deployment
    let deployment = expected
        .get("Deployment-nginx-main.json")
        .expect("Missing Deployment-nginx-main.json");
    assert_eq!(deployment["kind"], "Deployment");
    assert_eq!(deployment["metadata"]["name"], "nginx-main");
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["image"],
        "docker.io/library/nginx:1.27.3"
    );

    // Check Service
    let service = expected
        .get("Service-nginx-main.json")
        .expect("Missing Service-nginx-main.json");
    assert_eq!(service["kind"], "Service");
    assert_eq!(service["metadata"]["name"], "nginx-main");
    assert_eq!(service["spec"]["type"], "ClusterIP");
}

#[test]
fn test_podinfo_specific_structure() {
    let expected = load_expected_resources("podinfo");
    assert_eq!(expected.len(), 2);

    // Check Deployment
    let deployment = expected
        .get("Deployment-podinfo-main.json")
        .expect("Missing Deployment-podinfo-main.json");
    assert_eq!(deployment["kind"], "Deployment");
    assert_eq!(deployment["metadata"]["name"], "podinfo-main");
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["image"],
        "docker.io/stefanprodan/podinfo:6.7.1"
    );
    assert_eq!(
        deployment["spec"]["template"]["spec"]["containers"][0]["ports"][0]["containerPort"],
        9898
    );

    // Check Service
    let service = expected
        .get("Service-podinfo-main.json")
        .expect("Missing Service-podinfo-main.json");
    assert_eq!(service["kind"], "Service");
    assert_eq!(service["metadata"]["name"], "podinfo-main");
    assert_eq!(service["spec"]["ports"][0]["port"], 9898);
}

#[test]
fn test_full_example_specific_structure() {
    let expected = load_expected_resources("full");
    assert_eq!(expected.len(), 5);

    // Check all expected resource files exist
    assert!(expected.contains_key("Deployment-myapp-main.json"));
    assert!(expected.contains_key("Service-myapp-main.json"));
    assert!(expected.contains_key("ConfigMap-myapp-config.json"));
    assert!(expected.contains_key("ServiceAccount-myapp-main.json"));
    assert!(expected.contains_key("PersistentVolumeClaim-myapp-data.json"));

    // Check global labels are present on all resources
    for (filename, resource) in &expected {
        let labels = &resource["metadata"]["labels"];
        assert_eq!(
            labels["environment"], "production",
            "Missing/incorrect environment label in {}",
            filename
        );
        assert_eq!(
            labels["team"], "platform",
            "Missing/incorrect team label in {}",
            filename
        );

        let annotations = &resource["metadata"]["annotations"];
        assert_eq!(
            annotations["example.com/version"], "1.0.0",
            "Missing/incorrect version annotation in {}",
            filename
        );
    }
}

#[test]
fn test_resource_consistency_across_examples() {
    let examples = ["hello-world", "nginx", "podinfo", "full"];

    for example in &examples {
        let expected = load_expected_resources(example);

        for (filename, resource) in &expected {
            // Every resource should have basic required fields
            assert!(
                resource["apiVersion"].is_string(),
                "Missing apiVersion in {} resource from {}",
                filename,
                example
            );
            assert!(
                resource["kind"].is_string(),
                "Missing kind in {} resource from {}",
                filename,
                example
            );
            assert!(
                resource["metadata"]["name"].is_string(),
                "Missing metadata.name in {} resource from {}",
                filename,
                example
            );

            // Kubernetes resources should have proper labels
            let labels = &resource["metadata"]["labels"];
            if labels.is_object() {
                assert!(
                    labels["app.kubernetes.io/name"].is_string(),
                    "Missing app.kubernetes.io/name label in {} resource from {}",
                    filename,
                    example
                );
                assert!(
                    labels["app.kubernetes.io/instance"].is_string(),
                    "Missing app.kubernetes.io/instance label in {} resource from {}",
                    filename,
                    example
                );
            }
        }
    }
}

#[test]
fn test_name_prefix_functionality() {
    // Test that the --name flag correctly prefixes resource names
    let test_cases = [
        ("hello-world", "hello-world"),
        ("nginx", "nginx"),
        ("podinfo", "podinfo"),
        ("full", "myapp"),
    ];

    for (example_name, app_name) in &test_cases {
        let app_toml_path = format!("examples/{}/app.toml", example_name);
        let resources = process_values_file(&app_toml_path).unwrap();
        let prefixed_resources = apply_name_prefix(resources, app_name);

        for resource in &prefixed_resources {
            let resource_name = resource["metadata"]["name"].as_str().unwrap();
            assert!(
                resource_name.starts_with(&format!("{}-", app_name)),
                "Resource name '{}' should start with app name prefix '{}-' in example {}",
                resource_name,
                app_name,
                example_name
            );
        }
    }
}

#[test]
fn test_default_resource_naming() {
    // Test that resource names match their TOML keys
    let app_toml_path = "examples/nginx/app.toml";
    let resources = process_values_file(app_toml_path).unwrap();

    for resource in &resources {
        let labels = &resource["metadata"]["labels"];

        // For nginx example: controllers.main -> "main", service.main -> "main"
        // Resource names should be "main" (before CLI prefixing)
        assert_eq!(
            labels["app.kubernetes.io/name"], "main",
            "app.kubernetes.io/name label should be 'main' for nginx example"
        );
    }
}
