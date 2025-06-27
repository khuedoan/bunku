# Runtime requirements

## System Requirements

- **Rust**: 1.70 or later
- **Cargo**: Latest version with Rust installation

## Dependencies

The following crates are used:
- `k8s-openapi`: Kubernetes API types
- `gateway-api`: Gateway API types
- `serde`: Serialization/deserialization
- `toml`: TOML parsing
- `clap`: Command-line interface

## Kubernetes Cluster

- **Version**: Kubernetes v1.25 or later
- **APIs**: Core API v1 (for Deployments, Services, ConfigMaps, ServiceAccounts, PVCs)
- **Gateway API**: v1beta1 or later (for HTTPRoute support when enabled)

## Storage

- A default storage class that supports dynamic provisioning (for PersistentVolumeClaims)

## Output

- Generated manifests are in JSON format
- Compatible with `kubectl apply -f` or GitOps tools
- No special runtime dependencies for the generated manifests
