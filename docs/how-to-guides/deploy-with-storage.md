# Deploy with Persistent Storage

This guide shows you how to add persistent storage to your applications using PersistentVolumeClaims (PVCs).

## When to Use This Guide

- You need to persist data beyond pod restarts
- Your application requires shared storage between pods
- You want to separate storage configuration from application logic

## Prerequisites

- Kubernetes cluster with a storage provisioner
- Basic understanding of Kubernetes storage concepts
- Bunku installed and configured

## Basic Persistent Storage

Add a PVC to your `app.toml`:

```toml
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]
```

Then mount it in your container:

```toml
[controllers.main.containers.app]
image = "your-app:latest"

[[controllers.main.containers.app.volumeMounts]]
name = "data"
mountPath = "/app/data"
```

## Multiple Storage Volumes

For applications needing different types of storage:

```toml
# Fast SSD for database
[persistence.database]
enabled = true
type = "pvc"
size = "50Gi"
storageClass = "fast-ssd"
accessModes = ["ReadWriteOnce"]

# Slow storage for logs/backups
[persistence.logs]
enabled = true
type = "pvc"
size = "100Gi"
storageClass = "standard"
accessModes = ["ReadWriteOnce"]

# Shared storage for uploads
[persistence.uploads]
enabled = true
type = "pvc"
size = "200Gi"
storageClass = "shared"
accessModes = ["ReadWriteMany"]
```

Mount them in containers:

```toml
[controllers.main.containers.app]
image = "postgres:15"

[[controllers.main.containers.app.volumeMounts]]
name = "database"
mountPath = "/var/lib/postgresql/data"

[[controllers.main.containers.app.volumeMounts]]
name = "logs"
mountPath = "/var/log"

[[controllers.main.containers.app.volumeMounts]]
name = "uploads"
mountPath = "/shared/uploads"
```

## Storage Classes

Specify storage performance characteristics:

```toml
[persistence.data]
enabled = true
type = "pvc"
size = "20Gi"
storageClass = "fast-ssd"  # Use fast SSDs
accessModes = ["ReadWriteOnce"]

[persistence.backup]
enabled = true
type = "pvc"
size = "100Gi"
storageClass = "cold-storage"  # Use cheap, slow storage
accessModes = ["ReadWriteOnce"]
```

Common storage classes:
- `standard` - Default, balanced performance
- `fast-ssd` - High-performance SSD
- `cold-storage` - Cheap archival storage
- `shared` - Network-attached storage for multiple pods

## Access Modes

Choose the right access pattern:

```toml
# Single pod read-write (most common)
[persistence.database]
enabled = true
type = "pvc"
size = "50Gi"
accessModes = ["ReadWriteOnce"]

# Multiple pods read-only
[persistence.static-content]
enabled = true
type = "pvc"
size = "5Gi"
accessModes = ["ReadOnlyMany"]

# Multiple pods read-write (requires special storage)
[persistence.shared-data]
enabled = true
type = "pvc"
size = "20Gi"
accessModes = ["ReadWriteMany"]
```

## Complete Database Example

Here's a complete PostgreSQL deployment with persistent storage:

```toml
[global]

[controllers.database]
enabled = true
type = "deployment"
replicas = 1

[controllers.database.containers.postgres]
image = "postgres:15"

[controllers.database.containers.postgres.env.POSTGRES_DB]
value = "myapp"

[controllers.database.containers.postgres.env.POSTGRES_USER]
value = "admin"

[controllers.database.containers.postgres.env.POSTGRES_PASSWORD]
value = "changeme"  # Use proper secrets in production!

[controllers.database.containers.postgres.ports.postgres]
containerPort = 5432

[[controllers.database.containers.postgres.volumeMounts]]
name = "data"
mountPath = "/var/lib/postgresql/data"

[persistence.data]
enabled = true
type = "pvc"
size = "50Gi"
storageClass = "fast-ssd"
accessModes = ["ReadWriteOnce"]

[service.database]
enabled = true
type = "ClusterIP"
controller = "database"

[service.database.ports.postgres]
port = 5432
targetPort = 5432
```

## StatefulSet Storage

For databases and other stateful applications that need stable storage:

```toml
[controllers.database]
enabled = true
type = "statefulset"  # Instead of deployment
replicas = 3

[controllers.database.statefulset]
podManagementPolicy = "Parallel"

# Volume claim templates create PVCs automatically
[[controllers.database.statefulset.volumeClaimTemplates]]
enabled = true
size = "20Gi"
accessModes = ["ReadWriteOnce"]
storageClass = "fast-ssd"
```

## Troubleshooting

**PVC stuck in Pending:**
```bash
kubectl describe pvc your-pvc-name
# Check events for storage provisioning issues
```

**Permission denied in container:**
```toml
# Set security context for proper file permissions
[controllers.main.containers.app]
securityContext = { runAsUser = 1000, runAsGroup = 1000 }
```

**Storage class not found:**
```bash
# List available storage classes
kubectl get storageclass
```

**PVC won't delete:**
```bash
# Check if pods are still using the PVC
kubectl get pods -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.spec.volumes[*].persistentVolumeClaim.claimName}{"\n"}{end}'
```

## Best Practices

1. **Size appropriately** - Start small, you can usually expand later
2. **Use storage classes** - Don't rely on default storage
3. **Plan for backups** - Persistent storage needs backup strategies
4. **Test disaster recovery** - Verify you can restore from backups
5. **Monitor usage** - Watch for storage growing unexpectedly

## Next Steps

- [Configure Environment Variables](configure-environment.md) - Pass database URLs to your app
- [Set up Load Balancing](setup-load-balancing.md) - Expose your database applications
