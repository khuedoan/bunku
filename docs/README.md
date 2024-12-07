# `bunku`

**BUN**dle **KU**bernetes applications.

You provide an `app.toml` file (see [./examples](./examples)), and `bunku` will
generate an [OCI
image](https://github.com/opencontainers/image-spec/blob/main/spec.md) that
contains Kubernetes manifests, which can be used with GitOps controllers like
[ArgoCD](https://argo-cd.readthedocs.io) or [Flux CD](https://fluxcd.io/) to
apply to Kubernetes clusters.

[Continue reading](https://bunku,khuedoan.com) to learn more about the `app.toml` file format.
