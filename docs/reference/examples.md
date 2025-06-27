# Examples

Real-world Bunku configurations demonstrating common application patterns.

## Hello World

A minimal example to get started quickly.

**File: `hello-world.toml`**
```toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "docker.io/library/busybox:1.36"
command = ["/bin/sh"]
args = ["-c", "while true; do echo Hello World!; sleep 5; done"]
```

**Generate and Deploy:**
```bash
bunku --name hello --filename hello-world.toml --output-dir ./manifests
kubectl apply -f ./manifests/
kubectl logs -l app.kubernetes.io/name=main
```

**Generated Resources:**
- `Deployment-hello-main.json` - Single busybox container

## Web Server with Service

A complete web application with networking.

**File: `nginx.toml`**
```toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "docker.io/library/nginx:1.27.3"

[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
protocol = "TCP"
```

**Generate and Deploy:**
```bash
bunku --name nginx --filename nginx.toml --output-dir ./manifests
kubectl apply -f ./manifests/
kubectl port-forward service/nginx-main 8080:80
curl http://localhost:8080
```

**Generated Resources:**
- `Deployment-nginx-main.json` - nginx web server
- `Service-nginx-main.json` - ClusterIP service on port 80

## Application with Monitoring

A containerized application with health checks and monitoring endpoints.

**File: `podinfo.toml`**
```toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "docker.io/stefanprodan/podinfo:6.7.1"

[controllers.main.containers.app.ports.http]
containerPort = 9898
protocol = "TCP"

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 9898
targetPort = 9898
protocol = "TCP"
```

**Generate and Deploy:**
```bash
bunku --name podinfo --filename podinfo.toml --output-dir ./manifests
kubectl apply -f ./manifests/
kubectl port-forward service/podinfo-main 9898:9898

# Test the application
curl http://localhost:9898/healthz
curl http://localhost:9898/metrics
```

**Generated Resources:**
- `Deployment-podinfo-main.json` - podinfo demo application
- `Service-podinfo-main.json` - Service exposing metrics and health endpoints

## Full-Featured Application

A production-ready application with all resource types.

**File: `full.toml`**
```toml
[global]
labels."environment" = "production"
labels."team" = "platform"
annotations."example.com/version" = "1.0.0"

# Main application deployment
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "nginx:1.25"

[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"

# Service to expose the application
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
protocol = "TCP"

# Configuration data
[configMaps.config]
enabled = true

[configMaps.config.data]
"nginx.conf" = "server { listen 80; }"
"app.properties" = "debug=false"

# Service account for RBAC
[serviceAccount.main]
enabled = true

# Persistent storage
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]
```

**Generate and Deploy:**
```bash
bunku --name myapp --filename full.toml --output-dir ./manifests
kubectl apply -f ./manifests/
```

**Generated Resources:**
- `Deployment-myapp-main.json` - nginx deployment with 3 replicas
- `Service-myapp-main.json` - ClusterIP service
- `ConfigMap-myapp-config.json` - Configuration files
- `ServiceAccount-myapp-main.json` - Pod identity
- `PersistentVolumeClaim-myapp-data.json` - 10Gi storage

## Database with Persistent Storage

A PostgreSQL database with persistent storage and secrets.

**File: `postgres.toml`**
```toml
[global]
labels."app" = "postgres"

# Database deployment
[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.postgres]
image = "postgres:15"

[controllers.main.containers.postgres.env.POSTGRES_DB]
value = "myapp"

[controllers.main.containers.postgres.env.POSTGRES_USER]
value = "postgres"

[controllers.main.containers.postgres.env.POSTGRES_PASSWORD]
valueFrom.secretKeyRef.name = "postgres-secrets"
valueFrom.secretKeyRef.key = "password"

[controllers.main.containers.postgres.ports.postgres]
containerPort = 5432

[[controllers.main.containers.postgres.volumeMounts]]
name = "data"
mountPath = "/var/lib/postgresql/data"

# Database service
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.postgres]
port = 5432
targetPort = 5432

# Database credentials
[secrets.postgres-secrets]
enabled = true
type = "Opaque"

[secrets.postgres-secrets.stringData]
password = "change-me-in-production"

# Persistent storage for database
[persistence.data]
enabled = true
type = "pvc"
size = "50Gi"
accessModes = ["ReadWriteOnce"]
storageClass = "fast-ssd"
```

**Generate and Deploy:**
```bash
bunku --name postgres --filename postgres.toml --output-dir ./manifests
kubectl apply -f ./manifests/

# Connect to database
kubectl port-forward service/postgres-main 5432:5432
psql -h localhost -U postgres -d myapp
```

## Multi-Container Application

An application with main container, sidecar, and init container.

**File: `multi-container.toml`**
```toml
[global]

# Main application
[controllers.main]
enabled = true
type = "deployment"
replicas = 2

# Primary application container
[controllers.main.containers.app]
image = "myapp:v1.2.3"

[controllers.main.containers.app.ports.http]
containerPort = 8080

[controllers.main.containers.app.env.CONFIG_FILE]
value = "/etc/config/app.conf"

[[controllers.main.containers.app.volumeMounts]]
name = "config"
mountPath = "/etc/config"

# Sidecar logging container
[controllers.main.containers.logger]
image = "fluentd:v1.16"

[controllers.main.containers.logger.env.FLUENTD_CONF]
value = "fluentd.conf"

[[controllers.main.containers.logger.volumeMounts]]
name = "logs"
mountPath = "/var/log"

# Init container for database migration
[controllers.main.initContainers.migrate]
image = "myapp-migrate:v1.2.3"
command = ["./migrate"]
args = ["--database-url", "postgres://db:5432/myapp"]

# Service
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 8080

# Configuration
[configMaps.config]
enabled = true

[configMaps.config.data]
"app.conf" = '''
server_port = 8080
database_url = "postgres://db:5432/myapp"
log_level = "info"
'''

"fluentd.conf" = '''
<source>
  @type tail
  path /var/log/app.log
  format json
</source>
'''

# Storage for logs
[persistence.logs]
enabled = true
type = "pvc"
size = "5Gi"
accessModes = ["ReadWriteOnce"]
```

**Usage:**
```bash
bunku --name webapp --filename multi-container.toml --output-dir ./manifests
kubectl apply -f ./manifests/
```

## Microservices Setup

Multiple related services with shared configuration.

**File: `microservices.toml`**
```toml
[global]
labels."project" = "ecommerce"
labels."environment" = "staging"

# API Gateway
[controllers.api-gateway]
enabled = true
type = "deployment"
replicas = 2

[controllers.api-gateway.containers.nginx]
image = "nginx:1.27.3"

[controllers.api-gateway.containers.nginx.ports.http]
containerPort = 80

[service.api-gateway]
enabled = true
type = "LoadBalancer"
controller = "api-gateway"

[service.api-gateway.ports.http]
port = 80
targetPort = 80

# User Service
[controllers.users]
enabled = true
type = "deployment"
replicas = 3

[controllers.users.containers.app]
image = "users-service:v2.1.0"

[controllers.users.containers.app.ports.http]
containerPort = 8080

[controllers.users.containers.app.env.DATABASE_URL]
valueFrom.secretKeyRef.name = "database-secrets"
valueFrom.secretKeyRef.key = "users_db_url"

[service.users]
enabled = true
type = "ClusterIP"
controller = "users"

[service.users.ports.http]
port = 8080
targetPort = 8080

# Orders Service
[controllers.orders]
enabled = true
type = "deployment"
replicas = 3

[controllers.orders.containers.app]
image = "orders-service:v2.1.0"

[controllers.orders.containers.app.ports.http]
containerPort = 8080

[controllers.orders.containers.app.env.DATABASE_URL]
valueFrom.secretKeyRef.name = "database-secrets"
valueFrom.secretKeyRef.key = "orders_db_url"

[service.orders]
enabled = true
type = "ClusterIP"
controller = "orders"

[service.orders.ports.http]
port = 8080
targetPort = 8080

# Shared secrets
[secrets.database-secrets]
enabled = true
type = "Opaque"

[secrets.database-secrets.stringData]
users_db_url = "postgres://users:pass@users-db:5432/users"
orders_db_url = "postgres://orders:pass@orders-db:5432/orders"

# Shared configuration
[configMaps.shared-config]
enabled = true

[configMaps.shared-config.data]
log_level = "info"
metrics_enabled = "true"
tracing_endpoint = "http://jaeger:14268/api/traces"
```

**Deploy with environment-specific names:**
```bash
# Staging
bunku --name staging --filename microservices.toml --output-dir ./staging

# Production
bunku --name prod --filename microservices.toml --output-dir ./prod
```

## Scheduled Jobs

CronJob for periodic tasks like backups and maintenance.

**File: `scheduled-tasks.toml`**
```toml
[global]

# Daily backup job
[controllers.backup]
enabled = true
type = "cronjob"

[controllers.backup.cronjob]
schedule = "0 2 * * *"  # Daily at 2 AM
suspend = false
concurrencyPolicy = "Forbid"
successfulJobsHistory = 3
failedJobsHistory = 1

[controllers.backup.containers.backup]
image = "backup-tool:v1.0.0"
command = ["/backup.sh"]

[controllers.backup.containers.backup.env.BACKUP_TARGET]
value = "s3://my-backups/database"

[controllers.backup.containers.backup.env.DATABASE_URL]
valueFrom.secretKeyRef.name = "backup-secrets"
valueFrom.secretKeyRef.key = "database_url"

# Weekly cleanup job
[controllers.cleanup]
enabled = true
type = "cronjob"

[controllers.cleanup.cronjob]
schedule = "0 3 * * 0"  # Weekly on Sunday at 3 AM
suspend = false

[controllers.cleanup.containers.cleanup]
image = "cleanup-tool:v1.0.0"
command = ["/cleanup.sh"]

# Backup credentials
[secrets.backup-secrets]
enabled = true
type = "Opaque"

[secrets.backup-secrets.stringData]
database_url = "postgres://backup:secret@db:5432/myapp"
s3_access_key = "AKIAIOSFODNN7EXAMPLE"
s3_secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
```

## Development vs Production

Different configurations for different environments.

**File: `dev.toml`**
```toml
[global]
labels."environment" = "development"

[controllers.main]
enabled = true
type = "deployment"
replicas = 1  # Single replica for dev

[controllers.main.containers.app]
image = "myapp:latest"  # Latest for dev

[controllers.main.containers.app.env.LOG_LEVEL]
value = "debug"  # Debug logging

[controllers.main.containers.app.env.HOT_RELOAD]
value = "true"

[service.main]
enabled = true
type = "NodePort"  # NodePort for easy access
controller = "main"
```

**File: `prod.toml`**
```toml
[global]
labels."environment" = "production"
annotations."monitoring" = "enabled"

[controllers.main]
enabled = true
type = "deployment"
replicas = 5  # Multiple replicas for prod

[controllers.main.containers.app]
image = "myapp:v1.2.3"  # Specific version for prod

[controllers.main.containers.app.env.LOG_LEVEL]
value = "warn"  # Minimal logging

[controllers.main.containers.app.resources]
requests = { cpu = "100m", memory = "256Mi" }
limits = { cpu = "500m", memory = "512Mi" }

[service.main]
enabled = true
type = "ClusterIP"  # ClusterIP for internal access
controller = "main"

[persistence.data]
enabled = true
type = "pvc"
size = "100Gi"  # More storage for prod
storageClass = "fast-ssd"
```

**Deploy to different environments:**
```bash
# Development
bunku --name dev-myapp --filename dev.toml --output-dir ./dev
kubectl apply -f ./dev/ --namespace development

# Production
bunku --name prod-myapp --filename prod.toml --output-dir ./prod
kubectl apply -f ./prod/ --namespace production
```

## Working with the Examples

### Try the Examples

All examples are available in the project repository:

```bash
git clone https://github.com/khuedoan/bunku
cd bunku/examples

# Generate manifests for any example
bunku --name test --filename nginx/app.toml --output-dir ./test-manifests

# Apply to your cluster
kubectl apply -f ./test-manifests/
```

### Modify for Your Use Case

1. **Copy an example** closest to your use case
2. **Update the image** to your application
3. **Adjust resources** (replicas, memory, CPU)
4. **Add environment variables** your app needs
5. **Configure networking** (service type, ports)
6. **Add storage** if your app persists data

### Validation

Test your configuration before deploying:

```bash
# Check TOML syntax
bunku --filename your-app.toml > /dev/null

# Validate Kubernetes resources
bunku --filename your-app.toml | kubectl apply --dry-run=client -f -

# Generate and inspect
bunku --name test --filename your-app.toml --output-dir ./test
ls ./test/
cat ./test/Deployment-test-main.json | jq .
```

## Next Steps

- **[Getting Started Tutorial](../tutorials/getting-started.md)** - Step-by-step walkthrough
- **[TOML Configuration Reference](toml-config.md)** - Complete configuration options
- **[How-to Guides](../how-to-guides/)** - Solve specific problems
