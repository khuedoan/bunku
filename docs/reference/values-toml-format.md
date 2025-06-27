# values.toml format

The `values.toml` file uses TOML format to define Kubernetes resources. Below is the complete specification of supported configuration options.

## Global Configuration

```toml
[global]

[global.labels]
"app.kubernetes.io/version" = "1.0.0"
"environment" = "production"

[global.annotations]
"example.com/version" = "1.0.0"
```

## Controllers (Deployments)

```toml
[controllers.main]
enabled = true                        # Enable/disable this controller
type = "deployment"                   # Currently only "deployment" supported
replicas = 3                         # Number of replicas

[controllers.main.containers.app]
image = "nginx:1.25"                 # Container image

[controllers.main.containers.app.ports.http]
containerPort = 80                   # Port the container listens on
protocol = "TCP"                     # Protocol (TCP/UDP)
```

## Services

```toml
[service.main]
enabled = true                       # Enable/disable this service
type = "ClusterIP"                   # Service type (ClusterIP, NodePort, LoadBalancer)
controller = "main"                  # Which controller this service targets

[service.main.ports.http]
port = 80                           # Service port
targetPort = 80                     # Target port on pods
protocol = "TCP"                    # Protocol (TCP/UDP)
```

## ConfigMaps

```toml
[configMaps.config]
enabled = true                      # Enable/disable this ConfigMap

[configMaps.config.data]
"app.properties" = "debug=false"    # Key-value pairs for ConfigMap data
"config.yaml" = """
key: value
nested:
  key: value
"""
```

## ServiceAccounts

```toml
[serviceAccount.main]
enabled = true                      # Enable/disable this ServiceAccount
```

## Persistent Volume Claims

```toml
[persistence.data]
enabled = true                      # Enable/disable this PVC
type = "pvc"                       # Currently only "pvc" supported
size = "10Gi"                      # Storage size
accessModes = ["ReadWriteOnce"]    # Access modes
```
