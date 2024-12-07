# An overview of the app config file

The `app.toml` file is the core configuration file for `bunku`. It defines
workloads and their dependencies in a declarative, platform-agnostic, and
environment-agnostic manner.

Here's the general structure of an `app.toml` file:

```toml
apiVersion = "v1beta1"

[metadata]
name = "my-app"

[containers]
# Define application containers here

[service.ports]
# Define service ports here

# Define resources that the application depends on
[resources.redis]
type = "redis"
# ...
```
