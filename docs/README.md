# `bunku`

**BUN**dle **KU**bernetes applications.

You provide a TOML file (see [../examples](../examples)), and `bunku` will
generate Kubernetes manifests in JSON format to stdout that can be applied
directly to Kubernetes clusters with `kubectl apply -f -`.
Alternatively, it can push the output to an OCI registry, and you can use a
GitOps controller like [ArgoCD](https://argo-cd.readthedocs.io) or [Flux
CD](https://fluxcd.io/) to apply it - following the [rendered manifests
pattern](https://akuity.io/blog/the-rendered-manifests-pattern).

The tool supports generating the following Kubernetes resources:

- [x] `Deployment`
- [x] `Service`
- [x] `PersistentVolumeClaim`
- [x] `ServiceAccount`
- [x] `ConfigMap`
- [x] `HTTPRoute` (Gateway API)
- [ ] `SecretProviderClass`

Continue reading to learn more about the application file format (click
[here](https://bunku.khuedoan.com) to go to the documentation website if you're
reading this file directly).

## Acknowledgements

- [bjw's app-template Helm chart](https://bjw-s-labs.github.io/helm-charts/docs/app-template)
- [Rendered manifests pattern](https://akuity.io/blog/the-rendered-manifests-pattern)
- [Google Cloud Run Container runtime contract](https://cloud.google.com/run/docs/container-contract)
