# Set up Load Balancing

This guide shows you how to expose your applications to external traffic using LoadBalancer services and ingress controllers.

## When to Use This Guide

- You need to expose applications to users outside your cluster
- You want to handle multiple domains or paths
- You need SSL termination or advanced routing

## Prerequisites

- Bunku installed and configured
- Kubernetes cluster with external load balancer support OR ingress controller
- Basic understanding of Kubernetes networking

## LoadBalancer Service

The simplest way to expose an application externally:

```toml
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "nginx:1.27.3"

[controllers.main.containers.app.ports.http]
containerPort = 80

# LoadBalancer service for external access
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
protocol = "TCP"

# Optional: Request specific external IP
# loadBalancerIP = "203.0.113.1"
```

**Deploy and test:**
```bash
bunku --name webapp --filename app.toml --output-dir ./manifests
kubectl apply -f ./manifests/

# Get external IP (may take a few minutes)
kubectl get service webapp-main
```

## NodePort Service

For clusters without LoadBalancer support:

```toml
[service.main]
enabled = true
type = "NodePort"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
nodePort = 30080  # Port 30000-32767 range
```

**Access via any node:**
```bash
# Get node IP
kubectl get nodes -o wide

# Access application
curl http://<node-ip>:30080
```

## Multiple Port Applications

Expose applications with multiple ports:

```toml
[controllers.api]
enabled = true
type = "deployment"

[controllers.api.containers.app]
image = "myapi:v1.0.0"

# HTTP API port
[controllers.api.containers.app.ports.http]
containerPort = 8080

# gRPC port
[controllers.api.containers.app.ports.grpc]
containerPort = 9090

# Metrics port
[controllers.api.containers.app.ports.metrics]
containerPort = 8081

[service.api]
enabled = true
type = "LoadBalancer"
controller = "api"

# Expose all ports
[service.api.ports.http]
port = 80
targetPort = 8080

[service.api.ports.grpc]
port = 9090
targetPort = 9090

[service.api.ports.metrics]
port = 8081
targetPort = 8081
```

## Gateway API (HTTPRoute)

For advanced HTTP routing with Gateway API:

```toml
[controllers.main]
enabled = true
type = "deployment"

[controllers.main.containers.app]
image = "nginx:1.27.3"

[controllers.main.containers.app.ports.http]
containerPort = 80

# Internal service
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80

# HTTPRoute for external access
[httpRoutes.main]
enabled = true
parentRefs = [{ name = "gateway", namespace = "gateway-system" }]

[[httpRoutes.main.rules]]
[[httpRoutes.main.rules.matches]]
path = { type = "PathPrefix", value = "/" }
method = "GET"

[[httpRoutes.main.rules.backendRefs]]
name = "main"  # References service.main
port = 80
```

**Note:** HTTPRoute requires Gateway API CRDs and a compatible gateway controller.

## SSL/TLS Termination

### Using LoadBalancer with annotations

For cloud providers that support SSL through annotations:

```toml
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

# Cloud-specific annotations for SSL
[service.main.annotations]
"service.beta.kubernetes.io/aws-load-balancer-ssl-cert" = "arn:aws:acm:region:account:certificate/cert-id"
"service.beta.kubernetes.io/aws-load-balancer-ssl-ports" = "https"
"service.beta.kubernetes.io/aws-load-balancer-backend-protocol" = "http"

[service.main.ports.http]
port = 80
targetPort = 80

[service.main.ports.https]
port = 443
targetPort = 80
```

### Using TLS secrets

For ingress controllers that handle TLS:

```toml
# Create TLS secret
[secrets.tls-cert]
enabled = true
type = "kubernetes.io/tls"

[secrets.tls-cert.data]
"tls.crt" = "LS0tLS1CRUdJTi4uLi4="  # Base64 encoded certificate
"tls.key" = "LS0tLS1CRUdJTi4uLi4="  # Base64 encoded private key
```

## Complete Load Balanced Application

Here's a production-ready setup with load balancing:

```toml
[global]
labels."app" = "web-app"
labels."environment" = "production"

# Web application
[controllers.main]
enabled = true
type = "deployment"
replicas = 5  # Multiple replicas for HA

[controllers.main.containers.app]
image = "nginx:1.27.3"

[controllers.main.containers.app.ports.http]
containerPort = 80

# Health checks
[controllers.main.containers.app.livenessProbe]
httpGet = { path = "/health", port = 80 }
initialDelaySeconds = 30
periodSeconds = 10

[controllers.main.containers.app.readinessProbe]
httpGet = { path = "/ready", port = 80 }
initialDelaySeconds = 5
periodSeconds = 5

# Load balancer service
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80

[service.main.ports.https]
port = 443
targetPort = 80

# Cloud provider annotations
[service.main.annotations]
"service.beta.kubernetes.io/aws-load-balancer-type" = "nlb"
"service.beta.kubernetes.io/aws-load-balancer-cross-zone-load-balancing-enabled" = "true"

# Configuration
[configMaps.config]
enabled = true

[configMaps.config.data]
"nginx.conf" = '''
server {
    listen 80;
    location /health {
        return 200 "OK";
        add_header Content-Type text/plain;
    }
    location /ready {
        return 200 "Ready";
        add_header Content-Type text/plain;
    }
    location / {
        root /usr/share/nginx/html;
        index index.html;
    }
}
'''
```

## Multi-Environment Load Balancing

Different load balancing for different environments:

**Development (`dev.toml`):**
```toml
# Use NodePort for development
[service.main]
enabled = true
type = "NodePort"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
nodePort = 30080
```

**Staging (`staging.toml`):**
```toml
# Internal LoadBalancer for staging
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.annotations]
"service.beta.kubernetes.io/aws-load-balancer-internal" = "true"

[service.main.ports.http]
port = 80
targetPort = 80
```

**Production (`prod.toml`):**
```toml
# External LoadBalancer with SSL
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.annotations]
"service.beta.kubernetes.io/aws-load-balancer-ssl-cert" = "arn:aws:acm:region:account:certificate/prod-cert"
"service.beta.kubernetes.io/aws-load-balancer-ssl-ports" = "https"

[service.main.ports.http]
port = 80
targetPort = 80

[service.main.ports.https]
port = 443
targetPort = 80
```

## Cloud Provider Examples

### AWS Load Balancer

```toml
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.annotations]
# Use Network Load Balancer
"service.beta.kubernetes.io/aws-load-balancer-type" = "nlb"

# Enable cross-zone load balancing
"service.beta.kubernetes.io/aws-load-balancer-cross-zone-load-balancing-enabled" = "true"

# SSL certificate
"service.beta.kubernetes.io/aws-load-balancer-ssl-cert" = "arn:aws:acm:us-west-2:123456789:certificate/12345678-1234-1234-1234-123456789012"

# Backend protocol
"service.beta.kubernetes.io/aws-load-balancer-backend-protocol" = "http"

[service.main.ports.https]
port = 443
targetPort = 80
```

### Google Cloud Load Balancer

```toml
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.annotations]
# Static IP
"cloud.google.com/load-balancer-type" = "External"

# SSL certificate
"cloud.google.com/ssl-certificate" = "my-ssl-cert"

[service.main.ports.https]
port = 443
targetPort = 80
```

### Azure Load Balancer

```toml
[service.main]
enabled = true
type = "LoadBalancer"
controller = "main"

[service.main.annotations]
# Public IP
"service.beta.kubernetes.io/azure-load-balancer-resource-group" = "my-resource-group"

# DNS label
"service.beta.kubernetes.io/azure-dns-label-name" = "my-app"

[service.main.ports.http]
port = 80
targetPort = 80
```

## Troubleshooting

**LoadBalancer stuck in pending:**
```bash
kubectl describe service your-service-name
# Check events for provisioning issues
```

**Can't reach application:**
```bash
# Check if pods are ready
kubectl get pods -l app.kubernetes.io/name=main

# Check service endpoints
kubectl get endpoints your-service-name

# Test internal connectivity
kubectl run debug --image=busybox --rm -it -- wget -qO- http://your-service:80
```

**SSL certificate issues:**
```bash
# Check certificate status (AWS)
aws acm describe-certificate --certificate-arn your-cert-arn

# Check TLS secret
kubectl describe secret your-tls-secret
```

**Wrong external IP:**
```bash
# Check cloud provider annotations
kubectl describe service your-service-name

# Verify load balancer in cloud console
```

## Best Practices

1. **Use health checks** - Configure liveness and readiness probes
2. **Plan for scaling** - Use multiple replicas behind load balancers
3. **Secure traffic** - Always use HTTPS in production
4. **Monitor costs** - LoadBalancers can be expensive in cloud environments
5. **Test failover** - Verify behavior when pods restart
6. **Use appropriate service types** - ClusterIP → NodePort → LoadBalancer progression

## Next Steps

- [Configure Environment Variables](configure-environment.md) - Set up application configuration
- [Deploy with Persistent Storage](deploy-with-storage.md) - Add data persistence
