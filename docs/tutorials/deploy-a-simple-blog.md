# Deploy a simple blog

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
