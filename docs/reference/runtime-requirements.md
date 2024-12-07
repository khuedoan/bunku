# Runtime requirements

## Orchestrator

- Requires a Kubernetes cluster (v1.31 or later) to run the application.

## Networking

- Standard CNI
- A service mesh and gateway controller supporting at least the following:
    - mTLS
    - `Gateway`
    - `HTTPRoute`
- A default `Gateway` with a managed certificate (e.g., via [cert-manager](https://cert-manager.io))

## Secrets

- [External Secrets Operator](https://external-secrets.io)
- A default `ClusterSecretStore`

## Storage

- A default storage class that supports dynamic provisioning
