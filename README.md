# `bunku`

**BUN**dle **KU**bernetes applications.

You provide a [Score](https://docs.score.dev)-like file (see
[./examples](./examples)), and `bunku` will generate an [OCI
image](https://github.com/opencontainers/image-spec/blob/main/spec.md) that
contains Kubernetes manifests, which can be used with GitOps controllers like
[ArgoCD](https://argo-cd.readthedocs.io) or [Flux CD](https://fluxcd.io/) to
apply to Kubernetes clusters.

## Development

### Prerequisites

- Read the [Score specification reference](https://docs.score.dev/docs/score-specification/score-spec-reference)
- Install [Nix](https://nixos.org/download)

Open the development shell:

```sh
nix develop
```

## References

- [Score specification reference](https://docs.score.dev/docs/score-specification/score-spec-reference)
