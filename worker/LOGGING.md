# Worker Logging Configuration

The worker uses the `RUST_LOG` environment variable to control logging levels.

## Log Levels

- `error` - Only show errors
- `warn` - Show warnings and errors
- `info` - Show informational messages (default)
- `debug` - Show detailed debugging information
- `trace` - Show very detailed trace information

## Examples

### Default logging (info level)
```bash
cargo run
```

### Debug logging for worker only
```bash
RUST_LOG=worker=debug cargo run
```

### Debug logging for everything
```bash
RUST_LOG=debug cargo run
```

### Specific module debugging
```bash
RUST_LOG=worker::tasks::check_monitor=debug cargo run
```

### Multiple module configuration
```bash
RUST_LOG=worker=debug,graphile_worker=warn cargo run
```

### Production logging (errors only)
```bash
RUST_LOG=error cargo run
```

## Docker Usage

In docker-compose.yml:
```yaml
environment:
  - RUST_LOG=worker=info,graphile_worker=warn
```

Or when running docker:
```bash
docker run -e RUST_LOG=debug worker
```