# Configure Environment Variables

This guide shows you how to pass configuration to your applications using environment variables.

## When to Use This Guide

- You need to configure application behavior at runtime
- You want to use different settings for different environments (dev/staging/prod)
- Your application reads configuration from environment variables

## Prerequisites

- Bunku installed and configured
- Basic understanding of environment variables
- An application that accepts environment configuration

## Basic Environment Variables

Add environment variables directly in your TOML configuration:

```toml
[controllers.main.containers.app]
image = "your-app:latest"

[controllers.main.containers.app.env.DATABASE_URL]
value = "postgres://user:pass@db:5432/myapp"

[controllers.main.containers.app.env.LOG_LEVEL]
value = "info"

[controllers.main.containers.app.env.PORT]
value = "8080"
```

## Using ConfigMaps for Configuration

For larger configurations, use ConfigMaps:

```toml
# Define the ConfigMap
[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
database_url = "postgres://user:pass@db:5432/myapp"
redis_url = "redis://redis:6379/0"
log_level = "debug"
feature_flags = "new_ui,beta_api"
api_timeout = "30s"

# Reference ConfigMap values in environment variables
[controllers.main.containers.app]
image = "your-app:latest"

[controllers.main.containers.app.env.DATABASE_URL]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_url"

[controllers.main.containers.app.env.REDIS_URL]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "redis_url"

[controllers.main.containers.app.env.LOG_LEVEL]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "log_level"
```

## Environment-Specific Configuration

Create different TOML files for different environments:

**dev.toml:**
```toml
[global]
labels."environment" = "development"

[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
log_level = "debug"
database_url = "postgres://dev-user:dev-pass@dev-db:5432/myapp_dev"
feature_flags = "all_features"
```

**prod.toml:**
```toml
[global]
labels."environment" = "production"

[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
log_level = "warn"
database_url = "postgres://prod-user:secure-pass@prod-db:5432/myapp"
feature_flags = "stable_features"
```

Deploy to different environments:
```bash
# Development
bunku --name myapp-dev --filename dev.toml --output-dir ./dev-manifests

# Production
bunku --name myapp-prod --filename prod.toml --output-dir ./prod-manifests
```

## Configuration Files

Mount entire configuration files from ConfigMaps:

```toml
[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
"config.yaml" = '''
database:
  host: db.example.com
  port: 5432
  name: myapp

redis:
  host: redis.example.com
  port: 6379

logging:
  level: info
  format: json
'''

"nginx.conf" = '''
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://app:8080;
        proxy_set_header Host $host;
    }
}
'''

[controllers.main.containers.app]
image = "your-app:latest"

# Mount config files as volumes
[[controllers.main.containers.app.volumeMounts]]
name = "app-config"
mountPath = "/etc/app"
```

## Secrets for Sensitive Data

For passwords, API keys, and other sensitive data, use Kubernetes Secrets:

```toml
[secrets.app-secrets]
enabled = true
type = "Opaque"

[secrets.app-secrets.stringData]
database_password = "super-secret-password"
api_key = "sk-1234567890abcdef"
jwt_secret = "your-jwt-signing-secret"

[controllers.main.containers.app]
image = "your-app:latest"

# Reference secrets in environment variables
[controllers.main.containers.app.env.DATABASE_PASSWORD]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "database_password"

[controllers.main.containers.app.env.API_KEY]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "api_key"
```

## Complete Web Application Example

Here's a complete example for a web application with database:

```toml
[global]
labels."app" = "web-app"
labels."environment" = "production"

# Application secrets
[secrets.app-secrets]
enabled = true
type = "Opaque"

[secrets.app-secrets.stringData]
database_password = "change-me-in-production"
session_secret = "random-session-key"

# Application configuration
[configMaps.app-config]
enabled = true

[configMaps.app-config.data]
database_host = "postgres"
database_port = "5432"
database_name = "webapp"
database_user = "webapp_user"
redis_url = "redis://redis:6379/0"
log_level = "info"
max_connections = "100"

# Main application
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "your-webapp:v1.2.3"

# Environment variables from ConfigMap
[controllers.main.containers.app.env.DATABASE_HOST]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_host"

[controllers.main.containers.app.env.DATABASE_PORT]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_port"

[controllers.main.containers.app.env.DATABASE_NAME]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_name"

[controllers.main.containers.app.env.DATABASE_USER]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "database_user"

# Sensitive environment variables from Secret
[controllers.main.containers.app.env.DATABASE_PASSWORD]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "database_password"

[controllers.main.containers.app.env.SESSION_SECRET]
valueFrom.secretKeyRef.name = "app-secrets"
valueFrom.secretKeyRef.key = "session_secret"

[controllers.main.containers.app.env.REDIS_URL]
valueFrom.configMapKeyRef.name = "app-config"
valueFrom.configMapKeyRef.key = "redis_url"

[controllers.main.containers.app.ports.http]
containerPort = 8080

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 8080
```

## Best Practices

1. **Use ConfigMaps for non-sensitive data** - Configuration that can be shared
2. **Use Secrets for sensitive data** - Passwords, API keys, certificates
3. **Separate by environment** - Different configs for dev/staging/prod
4. **Use meaningful names** - `DATABASE_URL` instead of `DB_CONN`
5. **Document required variables** - List all environment variables your app needs
6. **Validate configuration** - Check for required variables at startup
7. **Use defaults** - Provide sensible defaults when possible

## Troubleshooting

**Container won't start:**
```bash
kubectl logs deployment/your-app
# Check for missing required environment variables
```

**ConfigMap not found:**
```bash
kubectl get configmap
kubectl describe configmap app-config
```

**Secret not found:**
```bash
kubectl get secret
kubectl describe secret app-secrets
```

**Wrong configuration values:**
```bash
# Check environment variables in running pod
kubectl exec -it pod-name -- env | grep APP_
```

## Next Steps

- [Deploy with Persistent Storage](deploy-with-storage.md) - Store application data
- [Set up Load Balancing](setup-load-balancing.md) - Expose your configured app externally
