# Why Bunku?

Understanding the design philosophy and goals behind Bunku's approach to Kubernetes application management.

## The Problem with Kubernetes Manifests

Kubernetes is powerful, but writing and maintaining raw YAML manifests is painful:

### Repetition and Boilerplate
```yaml
# Every deployment needs this boilerplate
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp
  labels:
    app.kubernetes.io/name: myapp
    app.kubernetes.io/instance: release
spec:
  selector:
    matchLabels:
      app.kubernetes.io/name: myapp
      app.kubernetes.io/instance: release
  template:
    metadata:
      labels:
        app.kubernetes.io/name: myapp
        app.kubernetes.io/instance: release
    spec:
      # Actual application config buried here
```

### Label Consistency Nightmares
Services must match deployments with exact label selectors. Get one label wrong and your service won't route traffic. Multiply this across dozens of resources and environments.

### Configuration Sprawl
A typical application might need:
- Deployment (pod template, containers, volumes)
- Service (networking, port mapping)
- ConfigMap (application config)
- Secret (passwords, API keys)
- PersistentVolumeClaim (storage)
- ServiceAccount (security)

That's 6+ YAML files, each with their own boilerplate and cross-references.

## Existing Solutions Fall Short

### Helm Charts: Complexity Overkill
Helm tries to solve this with templates, but introduces new problems:

```yaml
# Helm template - hard to read and debug
{{- if .Values.service.enabled }}
apiVersion: v1
kind: Service
metadata:
  name: {{ include "myapp.fullname" . }}
  labels:
    {{- include "myapp.labels" . | nindent 4 }}
spec:
  type: {{ .Values.service.type }}
  ports:
    {{- range .Values.service.ports }}
    - port: {{ .port }}
      targetPort: {{ .targetPort | default .port }}
      protocol: {{ .protocol | default "TCP" }}
      name: {{ .name }}
    {{- end }}
```

Problems with Helm:
- **Template complexity** - Hard to read, write, and debug
- **YAML in YAML** - Meta-templating creates cognitive overhead
- **Hidden logic** - Control flow scattered across helper templates
- **Testing difficulty** - Complex template rendering makes validation hard
- **Learning curve** - Need to understand Go templates AND Kubernetes

### Kustomize: Limited Scope
Kustomize handles patching and composition but doesn't reduce boilerplate:

```yaml
# Still need full YAML definitions
# Kustomize just patches them
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

resources:
- deployment.yaml
- service.yaml
- configmap.yaml

patchesStrategicMerge:
- patch-replica-count.yaml
- patch-environment.yaml
```

Problems with Kustomize:
- **Still verbose** - Full YAML definitions required
- **Patch complexity** - Strategic merge patches are error-prone
- **Limited abstraction** - No reduction in complexity

## The Bunku Philosophy

Bunku takes a different approach based on these principles:

### 1. Configuration, Not Templating

Instead of templating YAML, provide a purpose-built configuration format:

```toml
# Bunku - clear and concise
[controllers.main]
enabled = true
type = "deployment"
replicas = 3

[controllers.main.containers.app]
image = "nginx:1.27.3"

[controllers.main.containers.app.ports.http]
containerPort = 80

[service.main]
enabled = true
type = "ClusterIP"
controller = "main"

[service.main.ports.http]
port = 80
targetPort = 80
```

Benefits:
- **No templates** - Direct configuration mapping
- **Type safety** - Rust validates structure at generation time
- **Clear intent** - Configuration structure mirrors Kubernetes concepts
- **No hidden magic** - What you see is what you get

### 2. Smart Defaults and Relationships

Bunku automatically handles the tedious parts:

**Automatic Label Management:**
```toml
[service.main]
controller = "main"  # Automatically generates matching selectors
```

**Consistent Naming:**
```bash
bunku --name myapp --filename app.toml
# All resources get consistent "myapp-*" names
```

**Resource Relationships:**
- Services automatically target their specified controllers
- Volume mounts automatically reference persistence definitions
- Global labels and annotations apply everywhere

### 3. Kubernetes-Native Output

Generate standard Kubernetes JSON that works everywhere:

```bash
# Works with any Kubernetes cluster
bunku --filename app.toml | kubectl apply -f -

# Works with GitOps tools
bunku --name prod --filename app.toml --output-dir ./manifests
git add manifests/ && git commit -m "Update production"

# Works with CI/CD pipelines
bunku --name "${BRANCH}" --filename app.toml --output-dir ./deploy
kubectl apply -f ./deploy/ --namespace "${NAMESPACE}"
```

No special operators, controllers, or cluster modifications required.

### 4. Incremental Adoption

Start simple and add complexity as needed:

**Day 1: Basic deployment**
```toml
[controllers.main]
enabled = true

[controllers.main.containers.app]
image = "myapp:latest"
```

**Day 30: Add networking**
```toml
[service.main]
enabled = true
controller = "main"
```

**Day 90: Add persistence**
```toml
[persistence.data]
enabled = true
size = "10Gi"
```

Each addition is isolated and optional.

## Design Decisions

### Why TOML?

We chose TOML over YAML or JSON because:

**Readable Structure:**
```toml
[controllers.main.containers.app.env.DATABASE_URL]
value = "postgres://localhost:5432/myapp"
```

vs. YAML:
```yaml
controllers:
  main:
    containers:
      app:
        env:
          DATABASE_URL:
            value: "postgres://localhost:5432/myapp"
```

**No Indentation Errors:** TOML uses sections instead of indentation, eliminating common YAML pitfalls.

**Better for Configuration:** TOML was designed for configuration files, not data serialization.

### Why Rust?

Rust provides:
- **Type Safety** - Catch configuration errors at generation time
- **Performance** - Fast manifest generation even for large applications
- **Memory Safety** - No crashes on malformed input
- **Cross-platform** - Single binary works everywhere

### Why JSON Output?

JSON is more reliable than YAML for programmatic use:
- **Unambiguous parsing** - No YAML quirks or edge cases
- **Faster processing** - kubectl handles JSON faster than YAML
- **Better tooling** - More tools support JSON manipulation

## When to Use Bunku

### Good Fit
- **Microservices** - Many small, similar applications
- **Development environments** - Rapid iteration and testing
- **GitOps workflows** - Version-controlled infrastructure
- **Multi-environment deployments** - Same app, different configs
- **Teams new to Kubernetes** - Simpler than raw YAML

### Consider Alternatives
- **Complex operators** - Use operator-specific CRDs
- **Existing Helm investments** - Migration cost may not be worth it
- **Highly specialized workloads** - Raw YAML might be clearer
- **Team prefers YAML** - Don't force a tool change

## Comparison with Alternatives

| Tool | Complexity | Learning Curve | Type Safety | Output |
|------|------------|----------------|-------------|---------|
| **Raw YAML** | Low | Medium | None | Native |
| **Helm** | High | High | None | Native |
| **Kustomize** | Medium | Medium | None | Native |
| **Bunku** | Low | Low | Strong | Native |
| **Pulumi** | High | High | Strong | Non-native |
| **CDK8s** | High | High | Strong | Native |

### vs. Helm
- **Simpler** - No template language to learn
- **Faster** - Direct generation vs. template rendering
- **Safer** - Type checking vs. runtime template errors
- **Less features** - No package management or hooks

### vs. Kustomize
- **More opinionated** - Bunku makes decisions for you
- **Less flexible** - Can't patch arbitrary YAML
- **Easier** - Purpose-built vs. general patching tool
- **Better defaults** - Automatic relationships and naming

### vs. CDK8s/Pulumi
- **Domain-specific** - Built for Kubernetes, not general infrastructure
- **Simpler** - Configuration file vs. programming language
- **Faster iteration** - Edit config file vs. compile and run code
- **Less powerful** - No programming constructs

## The Future of Bunku

Bunku represents a middle path between raw YAML complexity and heavyweight templating solutions. It's designed for the 80% use case where you want:

- Simple, readable configuration
- Fast iteration cycles
- Strong validation
- Kubernetes-native output
- No operational overhead

For teams spending too much time fighting YAML but not ready for the complexity of Helm or CDK8s, Bunku provides a practical alternative that grows with your needs.

The goal isn't to replace all Kubernetes tooling, but to make the common case simple and the complex case possible.
