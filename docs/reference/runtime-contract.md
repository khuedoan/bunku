# Runtime contract

### Supported languages and images

- Supported languages will be determined by the platform's image builder.
- The only supported image format is OCI.

### Service port

- Supported protocols:
    - Plain `HTTP` (not `HTTPS`): TLS termination is managed by the runtime.
- Listen address: `0.0.0.0`.
- Listen port: The runtime provides a random port that the application can read
  from the `PORT` environment variable, or the user can specify a port number in
  the configuration.

### Environment variables

Default environment variables:

| Name   | Description             | Default                                            |
| ------ | ----------------------- | -------------------------------------------------- |
| `PORT` | The service port number | Assigned by the runtime or specified in `app.toml` |
