# Supported Resources

Bunku currently supports generating the following Kubernetes resources from your `values.toml` configuration:

## Deployments

- **Controllers**: Define application workloads with containers, replicas, and pod specifications
- **Configuration**: Supports container images, ports, environment variables, and resource limits
- **Features**: Rolling updates, replica scaling, pod templates

## Services

- **Types**: ClusterIP, NodePort, LoadBalancer
- **Configuration**: Port mappings, selectors, protocols
- **Features**: Automatic selector generation based on controller labels

## ConfigMaps

- **Data**: Key-value pairs for configuration files and environment variables
- **Formats**: Supports simple strings and multi-line TOML strings
- **Usage**: Configuration files, environment variables, command-line arguments

## ServiceAccounts

- **Authentication**: Pod identity for API server communication
- **Configuration**: Secret mounting, token auto-mounting
- **RBAC**: Works with Kubernetes Role-Based Access Control

## PersistentVolumeClaims

- **Storage**: Request persistent storage for applications
- **Access Modes**: ReadWriteOnce, ReadOnlyMany, ReadWriteMany
- **Configuration**: Size, storage class, access modes

## HTTPRoutes (Gateway API)

- **Status**: Defined but currently disabled due to import path issues
- **Purpose**: HTTP routing for Gateway API
- **Features**: Path-based routing, host matching, traffic splitting

## Resource Relationships

Bunku automatically manages relationships between resources:

- **Service → Deployment**: Services automatically target deployments using label selectors
- **Global Labels**: Applied to all generated resources
- **Global Annotations**: Applied to all generated resources
- **Naming**: Consistent naming across related resources

## Output Format

All resources are generated as:
- **JSON**: Kubernetes-compatible JSON manifests
- **Standards**: Follows Kubernetes API specifications using k8s-openapi crate
- **Validation**: Type-safe generation prevents invalid configurations
