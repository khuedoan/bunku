# CLI Reference

Complete reference for the `bunku` command-line interface.

## Synopsis

```bash
bunku [OPTIONS] --filename <FILENAME>
```

## Description

Bunku generates Kubernetes manifests from TOML configuration files. It produces
JSON manifests that are compatible with `kubectl apply`.

## Options

### Required Options

#### `--filename` / `-f`

- **Type**: String (file path)
- **Required**: Yes
- **Description**: Path to the TOML configuration file
- **Example**: `--filename app.toml`

### Optional Options

#### `--name` / `-n`

- **Type**: String
- **Required**: No
- **Description**: App name to prepend to all resource names. This allows
  deploying the same application multiple times with different names.
- **Example**: `--name my-app`
- **Result**: A controller named "main" becomes "my-app-main"

#### `--output-dir` / `-o`

- **Type**: Directory path
- **Required**: No
- **Description**: Output directory for separate JSON files. If not specified,
  outputs a JSON array to stdout.
- **Example**: `--output-dir ./manifests`
- **Behavior**: Creates one file per resource with naming pattern
  `{Kind}-{Name}.json`

#### `--help` / `-h`

- **Type**: Flag
- **Description**: Display help information and exit

#### `--version` / `-V`

- **Type**: Flag
- **Description**: Display version information and exit

## Output Formats

### Stdout Mode (Default)

When `--output-dir` is not specified:

```bash
bunku --filename app.toml
```

Outputs a JSON array containing all resources:

```json
[
  {
    "apiVersion": "apps/v1",
    "kind": "Deployment",
    "metadata": { "name": "my-app" },
    "spec": { ... }
  },
  {
    "apiVersion": "v1",
    "kind": "Service",
    "metadata": { "name": "my-app" },
    "spec": { ... }
  }
]
```

### File Mode

When `--output-dir` is specified:

```bash
bunku --name myapp --filename app.toml --output-dir ./manifests
```

Creates separate files:

```
./manifests/
├── Deployment-myapp-main.json
├── Service-myapp-main.json
```

## Security Considerations

### File Permissions

Ensure TOML files have appropriate permissions:

```bash
chmod 600 sensitive-config.toml  # Read-write for owner only
```

### Output Security

When using `--output-dir`, ensure the directory has appropriate permissions:

```bash
mkdir -p ./manifests
chmod 700 ./manifests  # Owner access only
```

### Secrets Handling

Never commit sensitive data to version control. Use Kubernetes Secrets and external
secret management systems.

## Troubleshooting

### Common Issues

**Resources not appearing:**
- Check the `enabled = true` setting in TOML
- Verify TOML syntax with a validator

**Wrong resource names:**
- Check the `--name` flag usage
- Verify TOML section names

**kubectl apply fails:**
- Test with `--dry-run=client` first
- Check cluster permissions
- Verify namespace exists

## See also

- [TOML Configuration](toml-config.md)
- [Examples](examples.md)
- [Getting Started Tutorial](../tutorials/getting-started.md)
