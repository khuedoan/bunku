# Bunku

**BUN**dle **KU**bernetes applications with simple TOML configuration.

Bunku is a Kubernetes manifest generator that converts TOML configuration files into
kubectl-ready JSON resources. It provides a simple, declarative way to define your
applications without complex templating.

## Quick Start

```bash
# Generate manifests to stdout
bunku --filename app.toml

# Generate separate files for kubectl apply
bunku --name myapp --filename app.toml --output-dir ./manifests

# Apply to Kubernetes
kubectl apply -f ./manifests/
```

## Features

- **Simple Configuration**: Write TOML instead of complex YAML templates
- **Type Safety**: Rust-based validation prevents invalid configurations
- **kubectl Compatible**: Generates standard Kubernetes JSON manifests
- **Flexible Output**: Stream to stdout or write separate files
- **Resource Relationships**: Automatic label selectors and naming consistency

## Supported Resources

- **Deployments** - Application workloads with containers and replicas
- **Services** - Network access with automatic label selectors
- **ConfigMaps** - Configuration data and files
- **ServiceAccounts** - Pod identity and RBAC integration
- **PersistentVolumeClaims** - Persistent storage requests
- **HTTPRoutes** - Gateway API routing (experimental)

## Documentation

This documentation follows the [Diátaxis framework](https://diataxis.fr/) to help
you find what you need:

### Tutorials

**Learn by doing** - Step-by-step lessons for beginners

- [Getting Started](tutorials/getting-started.md) - Your first bunku application
- [Deploy a Web Application](tutorials/deploy-web-application.md) - Complete
  example with storage and networking

### How-to Guides

**Solve specific problems** - Task-oriented instructions

- [Deploy with Persistent Storage](how-to-guides/deploy-with-storage.md)
- [Configure Environment Variables](how-to-guides/configure-environment.md)
- [Set up Load Balancing](how-to-guides/setup-load-balancing.md)
- [Contributing to Bunku](how-to-guides/contributing/)

### Reference

**Look up details** - Technical specifications and API documentation

- [CLI Reference](reference/cli.md) - Command-line options and usage
- [TOML Configuration](reference/toml-config.md) - Complete configuration reference
- [Supported Resources](reference/supported-resources.md) - Available Kubernetes
  resources
- [Examples](reference/examples.md) - Real-world configuration examples

### Explanation

**Understand concepts** - Background knowledge and design decisions

- [Why Bunku?](explanation/why-bunku.md) - Design philosophy and goals
- [Configuration Architecture](explanation/configuration-architecture.md) - How TOML
  maps to Kubernetes
- [Comparison with Helm](explanation/vs-helm.md) - When to use bunku vs other tools

## Installation

```bash
# From source (requires Rust)
cargo install --git https://github.com/khuedoan/bunku

# Or build locally
git clone https://github.com/khuedoan/bunku
cd bunku
cargo build --release
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE.md](../LICENSE.md)
for details.

## See also

- [bjw's app-template Helm chart](https://bjw-s-labs.github.io/helm-charts/docs/app-template)
- [Rendered manifests pattern](https://akuity.io/blog/the-rendered-manifests-pattern)
- [Google Cloud Run Container runtime contract](https://cloud.google.com/run/docs/container-contract)
