# Getting Started

Welcome to Bunku! This tutorial will guide you through creating your first Kubernetes application using Bunku's simple TOML configuration.

## What You'll Learn

By the end of this tutorial, you'll know how to:
- Write a basic `app.toml` configuration file
- Generate Kubernetes manifests with Bunku
- Deploy your application to a Kubernetes cluster

## Prerequisites

- Basic familiarity with Kubernetes concepts (pods, services, deployments)
- Access to a Kubernetes cluster (local or remote)
- `kubectl` installed and configured

## Step 1: Install Bunku

First, let's install Bunku. If you have Rust installed:

```bash
cargo install --git https://github.com/khuedoan/bunku
```

Or build from source:

```bash
git clone https://github.com/khuedoan/bunku
cd bunku
cargo build --release
# The binary will be at target/release/bunku
```

Verify the installation:

```bash
bunku --help
```

## Step 2: Create Your First Application

Let's create a simple "Hello World" application. Create a new file called `app.toml`:

```toml
# app.toml
[global]

[controllers.main]
enabled = true
type = "deployment"
replicas = 1

[controllers.main.containers.app]
image = "docker.io/library/busybox:1.36"
command = ["/bin/sh"]
args = ["-c", "while true; do echo Hello from Bunku!; sleep 10; done"]
```

This configuration defines:
- A **controller** (Deployment) named "main" with 1 replica
- A **container** using the busybox image that prints a message every 10 seconds

## Step 3: Generate Kubernetes Manifests

Now let's generate the Kubernetes manifests:

```bash
bunku --name hello-bunku --filename app.toml --output-dir ./manifests
```

This command:
- `--name hello-bunku`: Prefixes all resource names with "hello-bunku"
- `--filename app.toml`: Specifies your configuration file
- `--output-dir ./manifests`: Outputs separate JSON files instead of streaming to stdout

You should see:
```
Wrote ./manifests/Deployment-hello-bunku-main.json
```

Let's examine the generated file:

```bash
cat ./manifests/Deployment-hello-bunku-main.json
```

## Step 4: Deploy to Kubernetes

Apply the manifests to your Kubernetes cluster:

```bash
kubectl apply -f ./manifests/
```

Check that your deployment is running:

```bash
kubectl get deployments
kubectl get pods
```

You should see your `hello-bunku-main` deployment with 1 pod running.

## Step 5: View the Application Logs

See your application in action:

```bash
kubectl logs -l app.kubernetes.io/name=main
```

You should see the "Hello from Bunku!" message being printed every 10 seconds.

## Step 6: Clean Up

When you're done experimenting:

```bash
kubectl delete -f ./manifests/
```

## What's Next?

Congratulations! You've successfully:
- Created a TOML configuration file
- Generated Kubernetes manifests with Bunku  
- Deployed your application to Kubernetes

Now you're ready to explore more advanced features:

- **[Deploy a Web Application](deploy-web-application.md)** - Add networking and storage
- **[Configure Environment Variables](../how-to-guides/configure-environment.md)** - Pass configuration to your apps
- **[TOML Configuration Reference](../reference/toml-config.md)** - See all available options

## Troubleshooting

**Command not found**: Make sure Bunku is in your PATH or use the full path to the binary.

**Permission denied**: Ensure your kubectl context has permissions to create deployments in the current namespace.

**Pod not starting**: Check `kubectl describe pod <pod-name>` for detailed error messages.
