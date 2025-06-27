# Configuration Architecture

Understanding how Bunku maps TOML configuration to Kubernetes resources.

## Overview

Bunku uses a structured TOML configuration format that maps directly to Kubernetes
resources. This approach provides:

- Type safety through Rust's strong type system
- Simplified configuration through sensible defaults
- Automatic relationship management between resources
- Clear separation of configuration from implementation

## Configuration Structure

### Global Settings

The `[global]` section defines settings that apply to all generated resources:

```toml
[global]
# Labels and annotations for all resources
labels."app.kubernetes.io/version" = "1.0.0"
annotations."example.com/team" = "platform"

# Control label propagation
propagateGlobalMetadataToods = true
```

### Resource Sections

Each major section corresponds to a Kubernetes resource type:

```toml
[controllers.name]     # Deployments, StatefulSets
[service.name]        # Services
[configMaps.name]     # ConfigMaps
[secrets.name]        # Secrets
[persistence.name]    # PersistentVolumeClaims
```

## Resource Relationships

### Automatic Label Selection

Bunku automatically manages relationships between resources using consistent
labeling:

1. Each controller gets a unique set of labels
2. Services automatically target their controllers
3. Volumes are automatically mounted in the right containers

Example:

```toml
# Controller defines base labels
[controllers.web]
enabled = true
type = "deployment"

# Service automatically uses controller's labels
[service.web]
enabled = true
controller = "web"  # Links to controllers.web

# Storage is mounted where needed
[persistence.data]
enabled = true
size = "10Gi"

[[controllers.web.containers.app.volumeMounts]]
name = "data"  # Links to persistence.data
mountPath = "/data"
```

## Type System

### Strong Types

Bunku uses Rust's type system to validate configuration:

- Resource types must be valid (`deployment`, `statefulset`, etc.)
- Port numbers must be valid integers in range
- Memory/CPU quantities must use valid formats
- Required fields cannot be omitted

### Default Values

Sensible defaults reduce configuration complexity:

```toml
[controllers.main]
type = "deployment"    # Default type
replicas = 1          # Default replicas
enabled = true        # Default enabled

[service.main]
type = "ClusterIP"    # Default type
protocol = "TCP"      # Default protocol
```

## Configuration to Kubernetes Mapping

### Resource Generation

Each TOML section maps to specific Kubernetes resources:

1. `controllers.*` → Deployment, StatefulSet, etc.
2. `service.*` → Service
3. `configMaps.*` → ConfigMap
4. `secrets.*` → Secret
5. `persistence.*` → PersistentVolumeClaim

### Naming Convention

Resources follow a consistent naming pattern:

```
{app-name}-{resource-key}
```

Where:
- `app-name` comes from the `--name` CLI flag
- `resource-key` is the TOML section key

Example with `--name myapp`:
- `controllers.web` → `myapp-web`
- `service.web` → `myapp-web`

## Best Practices

### Configuration Organization

1. Group related resources together
2. Use consistent naming across sections
3. Leverage defaults for simpler configs
4. Split large configs into multiple files

### Resource Relationships

1. Use meaningful section names
2. Keep service names aligned with controllers
3. Use volume names that describe their purpose
4. Group related environment variables

## See also

- [TOML Configuration Reference](../reference/toml-config.md)
- [Examples](../reference/examples.md)
- [Why Bunku?](why-bunku.md)
