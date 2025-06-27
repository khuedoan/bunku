# Deploy a Web Application

In this tutorial, you'll build a complete web application with persistent storage, networking, and configuration. We'll deploy a simple nginx-based blog with custom content.

## What You'll Build

- **Web server** running nginx with custom content
- **Service** to expose the application within the cluster
- **ConfigMap** to store HTML content and nginx configuration
- **PersistentVolumeClaim** for log storage
- **ServiceAccount** for proper security

## Prerequisites

- Completed the [Getting Started](getting-started.md) tutorial
- Kubernetes cluster with storage provisioner
- Basic understanding of nginx and HTML

## Step 1: Plan the Application

Let's design our blog application architecture:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   ConfigMap     │────▶│   Deployment    │────▶│     Service     │
│  (HTML + Config)│     │  (nginx blog)   │     │  (networking)   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │      PVC        │
                        │  (log storage)  │
                        └─────────────────┘
```

## Step 2: Create the Configuration

Create a new file called `blog.toml`:

```toml
# blog.toml
[global]
labels."environment" = "tutorial"
labels."app" = "blog"

annotations."tutorial" = "web-application"

# Main web server deployment
[controllers.main]
enabled = true
type = "deployment"
replicas = 2

[controllers.main.containers.nginx]
image = "docker.io/library/nginx:1.27.3"

# Expose HTTP port
[controllers.main.containers.nginx.ports.http]
containerPort = 80
protocol = "TCP"

# Mount configuration and storage
[[controllers.main.containers.nginx.volumeMounts]]
name = "content"
mountPath = "/usr/share/nginx/html"

[[controllers.main.containers.nginx.volumeMounts]]
name = "logs"
mountPath = "/var/log/nginx"

# Service to expose the application
[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
protocol = "TCP"

# ConfigMap with HTML content and nginx config
[configMaps.content]
enabled = true

[configMaps.content.data]
"index.html" = '''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Bunku Blog</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #2c3e50; }
        .post { margin: 20px 0; padding: 20px; border-left: 4px solid #3498db; }
        .date { color: #7f8c8d; font-size: 0.9em; }
    </style>
</head>
<body>
    <h1>Welcome to My Bunku Blog!</h1>
    <div class="post">
        <h2>Getting Started with Kubernetes</h2>
        <p class="date">Published: Today</p>
        <p>This blog is powered by Bunku - a simple way to generate Kubernetes manifests from TOML configuration!</p>
        <p>Bunku makes it easy to deploy applications without complex templating.</p>
    </div>
    <div class="post">
        <h2>Why I Love Simple Tools</h2>
        <p class="date">Published: Yesterday</p>
        <p>Sometimes the best solutions are the simplest ones. Bunku proves that you don't need complex tools to deploy applications effectively.</p>
    </div>
</body>
</html>
'''

"about.html" = '''
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>About - My Bunku Blog</title>
</head>
<body>
    <h1>About This Blog</h1>
    <p>This is a demo blog created using Bunku to showcase its capabilities.</p>
    <p><a href="/">← Back to Home</a></p>
</body>
</html>
'''

# Persistent storage for logs
[persistence.logs]
enabled = true
type = "pvc"
size = "1Gi"
accessModes = ["ReadWriteOnce"]

# Service account for security
[serviceAccount.main]
enabled = true
```

This configuration is more complex than our hello-world example. Let's break it down:

- **Global section**: Adds common labels and annotations to all resources
- **Controller**: nginx deployment with 2 replicas and volume mounts
- **Service**: Exposes the web server on port 80
- **ConfigMap**: Contains our HTML files
- **PVC**: Persistent storage for nginx logs
- **ServiceAccount**: Provides pod identity

## Step 3: Generate and Deploy

Generate the manifests:

```bash
bunku --name my-blog --filename blog.toml --output-dir ./blog-manifests
```

You should see several files created:

```bash
ls -la blog-manifests/
```

Expected output:
```
Deployment-my-blog-main.json
Service-my-blog-main.json
ConfigMap-my-blog-content.json
PersistentVolumeClaim-my-blog-logs.json
ServiceAccount-my-blog-main.json
```

Deploy to Kubernetes:

```bash
kubectl apply -f ./blog-manifests/
```

## Step 4: Test the Application

Check that everything is running:

```bash
kubectl get all
kubectl get configmap,pvc
```

Test the application by port-forwarding to the service:

```bash
kubectl port-forward service/my-blog-main 8080:80
```

Open http://localhost:8080 in your browser. You should see your blog!

Try the about page: http://localhost:8080/about.html

## Step 5: Explore the Generated Resources

Let's examine what Bunku created:

**Deployment with volume mounts:**
```bash
kubectl describe deployment my-blog-main
```

**Service with label selectors:**
```bash
kubectl describe service my-blog-main
```

**ConfigMap with HTML content:**
```bash
kubectl get configmap my-blog-content -o yaml
```

**Persistent storage:**
```bash
kubectl describe pvc my-blog-logs
```

## Step 6: Make Changes

Let's update the blog content. Edit `blog.toml` and add a new post to the HTML:

```toml
[configMaps.content.data]
"index.html" = '''
<!DOCTYPE html>
<html lang="en">
<head>
    <!-- ... existing head content ... -->
</head>
<body>
    <h1>Welcome to My Bunku Blog!</h1>

    <!-- Add this new post -->
    <div class="post">
        <h2>Updated with Bunku!</h2>
        <p class="date">Published: Just now</p>
        <p>I just updated my blog by changing the TOML file and redeploying. So easy!</p>
    </div>

    <!-- ... existing posts ... -->
</body>
</html>
'''
```

Regenerate and apply:

```bash
bunku --name my-blog --filename blog.toml --output-dir ./blog-manifests
kubectl apply -f ./blog-manifests/
```

The ConfigMap will be updated, and Kubernetes will make the new content available to your pods.

## Step 7: Scale the Application

Need more capacity? Update the replicas in `blog.toml`:

```toml
[controllers.main]
enabled = true
type = "deployment"
replicas = 5  # Increased from 2
```

Redeploy:

```bash
bunku --name my-blog --filename blog.toml --output-dir ./blog-manifests
kubectl apply -f ./blog-manifests/
```

Watch the scaling:

```bash
kubectl get pods -w
```

## Step 8: Clean Up

When you're finished:

```bash
kubectl delete -f ./blog-manifests/
```

## What You've Learned

Congratulations! You've successfully:

- **Deployed a multi-component application** with 5 different Kubernetes resources
- **Used persistent storage** for application data  
- **Configured networking** to expose your application
- **Managed configuration** with ConfigMaps
- **Applied security best practices** with ServiceAccounts
- **Scaled your application** by changing configuration
- **Updated your application** without downtime

## Next Steps

You're now ready for production-grade applications! Continue with:

- **[Deploy with Persistent Storage](../how-to-guides/deploy-with-storage.md)** - Advanced storage patterns
- **[Set up Load Balancing](../how-to-guides/setup-load-balancing.md)** - External traffic routing
- **[Configure Environment Variables](../how-to-guides/configure-environment.md)** - Runtime configuration

Or explore the complete configuration reference:
- **[TOML Configuration](../reference/toml-config.md)** - All available options
- **[Examples](../reference/examples.md)** - Real-world configurations
