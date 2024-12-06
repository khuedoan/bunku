# `bunku`

**BUN**dle **KU**bernetes applications.

You provide a [Score](https://docs.score.dev)-like file (see
[./examples](./examples)), and `bunku` will generate an [OCI
image](https://github.com/opencontainers/image-spec/blob/main/spec.md) that
contains Kubernetes manifests, which can be used with GitOps controllers like
[ArgoCD](https://argo-cd.readthedocs.io) or [Flux CD](https://fluxcd.io/) to
apply to Kubernetes clusters.

## Tutorial

Let's start with a single container in an `app.toml` file located at the root
of your project:

```toml
apiVersion = "v1beta1"

[metadata]
name = "blog"

[containers.app]
image = "blog"
```

This configuration creates a container, but it won’t be accessible yet. To
allow other internal services to communicate with it, we need to define a
service:

```toml
[service.ports.http]
port = 3000
```

With this configuration, any service running within the same internal network
can access the blog. However, since we’re writing a blog for human to access,
we need to expose it to the public internet. To do this, let’s request a domain
and route traffic from the domain to our service:

```toml
[resources.dns]
type = "dns"
params = { subdomain = "blog" }

[resources.route]
type = "route"
params = { host = "${resources.dns.host}", port = "http" }
```

The base domain will be provided by your platform provider, so you only need to
specify the subdomain. The `${resources.dns.host}` placeholder represents a
resource output that will be replaced with the actual domain name provisioned
by the platform provider.

Now you can access your blog at `https://blog.<base-domain>`.

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
- [Placeholder parser inspired by `envsubst`](https://github.com/coreos/envsubst-rs)
