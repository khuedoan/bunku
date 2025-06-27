# An overview of the values config file

The `values.toml` file is the core configuration file for `bunku`. It defines
Kubernetes resources in a declarative manner using a structure similar to Helm charts.

Here's the general structure of a `values.toml` file:

```toml
# Global configuration applied to all resources
[global]

[global.labels]
"environment" = "production"

[global.annotations]
"version" = "1.0.0"

# Controllers (Deployments)
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "nginx:1.25"

[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"

# Services
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80

# ConfigMaps
[configMaps.config]
enabled = true

[configMaps.config.data]
"nginx.conf" = "server { listen 80; }"

# ServiceAccounts
[serviceAccount.main]
enabled = true

# Persistent Volume Claims
[persistence.data]
enabled = true
type = "pvc"
size = "10Gi"
accessModes = ["ReadWriteOnce"]
```

The configuration is organized into sections for each type of Kubernetes resource that bunku supports:

- **global**: Configuration applied to all resources (labels, annotations, naming)
- **controllers**: Deployments with containers, ports, and pod specifications
- **service**: Services with ports and selectors
- **configMaps**: ConfigMaps with data fields
- **serviceAccount**: ServiceAccounts with secrets and automount settings
- **persistence**: PersistentVolumeClaims with storage requirements
