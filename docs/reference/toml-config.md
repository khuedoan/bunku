# TOML Configuration Reference

Complete reference for the `app.toml` configuration file format.

## File Structure

Bunku uses TOML (Tom's Obvious, Minimal Language) for configuration. The file is
organized into sections that define different types of Kubernetes resources.

```toml
[global]                    # Global settings applied to all resources
[controllers.name]          # Deployments, StatefulSets, Jobs
[service.name]             # Services for network access
[configMaps.name]          # Configuration data
[secrets.name]             # Sensitive data
[persistence.name]         # PersistentVolumeClaims
[serviceAccount.name]      # Pod identity and RBAC
```

## Global Configuration

The `[global]` section contains settings applied to all generated resources:

```toml
[global]
propagateGlobalMetadataToods = true  # Apply global labels/annotations to pods

[global.labels]
"app.kubernetes.io/version" = "1.0.0"
"environment" = "production"
"team" = "platform"

[global.annotations]
"example.com/version" = "1.0.0"
"contact" = "platform-team@company.com"
```

### Global Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `propagateGlobalMetadataToods` | boolean | `false` | Apply global labels/annotations to pod templates |
| `labels` | object | `{}` | Labels applied to all resources |
| `annotations` | object | `{}` | Annotations applied to all resources |

## Controllers (Deployments)

Controllers define your application workloads:

```toml
[controllers.main]
enabled = true
type = "deployment"
replicas = 3
revisionHistoryLimit = 5

# Container definitions
[controllers.main.containers.app]
image = "nginx:1.27.3"
command = ["/usr/sbin/nginx"]
args = ["-g", "daemon off;"]

# Container ports
[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"

# Environment variables
[controllers.main.containers.app.env.LOG_LEVEL]
value = "info"

[controllers.main.containers.app.env.DATABASE_URL]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_url"
```

### Controller Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this controller |
| `type` | string | `"deployment"` | Controller type (deployment, statefulset) |
| `replicas` | integer | `1` | Number of pod replicas |
| `revisionHistoryLimit` | integer | `10` | Number of old ReplicaSets to retain |

### Container Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `image` | string | Required | Container image with tag |
| `command` | array | `[]` | Container entrypoint override |
| `args` | array | `[]` | Container command arguments |
| `env` | object | `{}` | Environment variables |
| `ports` | object | `{}` | Container port mappings |
| `volumeMounts` | array | `[]` | Volume mount points |

## Services

Services expose your applications on the network:

```toml
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 8080
protocol = "TCP"
```

### Service Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this service |
| `type` | string | `"ClusterIP"` | Service type |
| `controller` | string | Required | Target controller name |
| `ports` | object | `{}` | Service port mappings |

## ConfigMaps

ConfigMaps store non-sensitive configuration data:

```toml
[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
"config.json" = '''
{
  "database": {
    "host": "postgres",
    "port": 5432
  }
}
'''

"settings.conf" = '''
log_level = "info"
max_connections = 100
'''
```

### ConfigMap Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this ConfigMap |
| `data` | object | `{}` | Key-value configuration data |

## Secrets

Secrets store sensitive data like passwords and API keys:

```toml
[secrets.app-secrets]
enabled = true
type = "Opaque"

[secrets.app-secrets.stringData]
database_password = "super-secret-password"
api_key = "sk-1234567890abcdef"
jwt_secret = "your-jwt-signing-secret"

# For TLS certificates
[secrets.tls-cert]
enabled = true
type = "kubernetes.io/tls"

[secrets.tls-cert.data]
"tls.crt" = "LS0tLS1CRUdJTi..."  # base64-encoded certificate
"tls.key" = "LS0tLS1CRUdJTi..."  # base64-encoded private key
```

### Secret Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this Secret |
| `type` | string | `"Opaque"` | Secret type |
| `data` | object | `{}` | Base64-encoded data |
| `stringData` | object | `{}` | Plain text data (automatically encoded) |

### Common Secret Types

- `Opaque` - Generic secret data
- `kubernetes.io/tls` - TLS certificates
- `kubernetes.io/basic-auth` - Basic authentication
- `kubernetes.io/ssh-auth` - SSH authentication

## Persistent Storage

PersistentVolumeClaims request storage for your applications:

```toml
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]
storageClass = "fast-ssd"

# Volume mounts are defined in controller containers
[[controllers.main.containers.app.volumeMounts]]
name = "data"  # References persistence.data
mountPath = "/app/data"
```

### Persistence Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this PVC |
| `type` | string | `"pvc"` | Storage type (currently only PVC supported) |
| `size` | string | No | Storage size (e.g., "10Gi") |
| `accessModes` | array | `["ReadWriteOnce"]` | Access modes |
| `storageClass` | string | No | Storage class name |

### Access Modes

- `ReadWriteOnce` - Single node read-write
- `ReadOnlyMany` - Multiple nodes read-only
- `ReadWriteMany` - Multiple nodes read-write

## ServiceAccounts

ServiceAccounts provide pod identity for RBAC:

```toml
[serviceAccount.main]
enabled = true
automountServiceAccountToken = true

[serviceAccount.main.annotations]
"eks.amazonaws.com/role-arn" = "arn:aws:iam::123456789:role/my-role"

# Reference in controller
[controllers.main]
serviceAccount.identifier = "main"  # References serviceAccount.main
```

### ServiceAccount Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable this ServiceAccount |
| `automountServiceAccountToken` | boolean | `false` | Auto-mount service account token |
| `annotations` | object | `{}` | ServiceAccount annotations |
| `labels` | object | `{}` | ServiceAccount labels |

## Advanced Configurations

### StatefulSets

For applications requiring stable network identity and storage:

```toml
[controllers.database]
enabled = true
type = "statefulset"
replicas = 3

[controllers.database.statefulset]
podManagementPolicy = "Parallel"
serviceName = "database-headless"

# Volume claim templates
[[controllers.database.statefulset.volumeClaimTemplates]]
enabled = true
size = "20Gi"
accessModes = ["ReadWriteOnce"]
storageClass = "fast-ssd"
```

### Jobs and CronJobs

For one-time and scheduled tasks:

```toml
# One-time job
[controllers.migration]
enabled = true
type = "job"

[controllers.migration.job]
parallelism = 1
completions = 1
backoffLimit = 3
ttlSecondsAfterFinished = 300

# Scheduled job
[controllers.backup]
enabled = true
type = "cronjob"

[controllers.backup.cronjob]
schedule = "0 2 * * *"  # Daily at 2 AM
suspend = false
concurrencyPolicy = "Forbid"
successfulJobsHistory = 3
failedJobsHistory = 1
```

## Environment Variables

### Direct Values

```toml
[controllers.main.containers.app.env.LOG_LEVEL]
value = "info"

[controllers.main.containers.app.env.PORT]
value = "8080"
```

### ConfigMap References

```toml
[controllers.main.containers.app.env.DATABASE_HOST]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_host"
```

### Secret References

```toml
[controllers.main.containers.app.env.DATABASE_PASSWORD]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "database_password"
```

### Field References

```toml
[controllers.main.containers.app.env.NODE_NAME]
valueFrom.fieldRef.fieldPath = "spec.nodeName"

[controllers.main.containers.app.env.POD_IP]
valueFrom.fieldRef.fieldPath = "status.podIP"
```

## Volume Mounts

Link persistent storage to containers:

```toml
# Define storage
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"

# Mount in container
[[controllers.main.containers.app.volumeMounts]]
name = "data"          # Must match persistence key
mountPath = "/app/data"
readOnly = false
subPath = "app-data"   # Optional subdirectory
```

## Complete Example

Here's a comprehensive example combining all features:

```toml
[global]
labels."environment" = "production"
labels."team" = "platform"
annotations."contact" = "platform@company.com"

# Main application
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "myapp:v1.2.3"

[controllers.main.containers.app.ports.http]
containerPort = 8080
protocol = "TCP"

[controllers.main.containers.app.env.DATABASE_HOST]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_host"

[controllers.main.containers.app.env.DATABASE_PASSWORD]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "database_password"

[[controllers.main.containers.app.volumeMounts]]
name = "data"
mountPath = "/app/data"

# Service
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 8080

# Configuration
[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
database_host = "postgres.default.svc.cluster.local"
database_port = "5432"
log_level = "info"

# Secrets
[secrets.app-secrets]
enabled = true
type = "Opaque"

[secrets.app-secrets.stringData]
database_password = "change-me-in-production"

# Storage
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]

# Service account
[serviceAccount.main]
enabled = true
```

## Best Practices

1. **Use meaningful names** - `database` instead of `db1`
2. **Enable selectively** - Set `enabled = false` for unused resources
3. **Separate environments** - Use different TOML files per environment
4. **Validate syntax** - Use a TOML validator before deploying
5. **Use Secrets for sensitive data** - Never put passwords in ConfigMaps
6. **Document your configuration** - Add comments explaining complex sections

## Validation

Validate your TOML syntax:

```bash
# Check TOML syntax
bunku --filename app.toml > /dev/null

# Validate against Kubernetes
bunku --filename app.toml | kubectl apply --dry-run=client -f -
```

## See also

- [Getting Started Tutorial](../tutorials/getting-started.md)
- [Examples](examples.md)
- [CLI Reference](cli.md)
