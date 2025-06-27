# Deploy a simple application

Let's start with a simple web application deployment using a `values.toml` file.

## Basic Deployment

First, create a `values.toml` file with a basic deployment:

```toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "nginx:1.25"

[controllers.main.containers.app.ports.http]
containerPort = 80
protocol = "TCP"
```

This creates a deployment with a single nginx container, but it won't be accessible yet.

## Adding a Service

To allow other services or external traffic to communicate with your application, add a service:

```toml
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
protocol = "TCP"
```

## Adding Configuration

You can add configuration files using ConfigMaps:

```toml
[configMaps.config]
enabled = true

[configMaps.config.data]
"index.html" = """
<!DOCTYPE html>
<html>
<head><title>My Blog</title></head>
<body><h1>Welcome to my blog!</h1></body>
</html>
"""
```

## Complete Example

Here's the complete `values.toml` file:

```toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 2

[controllers.main.containers.app]
image = "nginx:1.25"

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

[configMaps.config]
enabled = true

[configMaps.config.data]
"index.html" = """
<!DOCTYPE html>
<html>
<head><title>My Blog</title></head>
<body><h1>Welcome to my blog!</h1></body>
</html>
"""
```

## Generate Manifests

Run bunku to generate the Kubernetes manifests:

```bash
bunku values.toml
```

This will output JSON manifests for a Deployment, Service, and ConfigMap that you can apply to your Kubernetes cluster.
