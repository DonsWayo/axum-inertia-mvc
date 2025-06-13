# Terraform Provider for RustGenie

This Terraform provider allows you to manage monitors in the RustGenie application using Infrastructure as Code.

## Features

- ✅ Create, Read, Update, and Delete monitors
- ✅ Support for different monitor types (HTTP, TCP, etc.)
- ✅ Data source to read existing monitors
- ✅ Configurable endpoint and optional API key authentication

## Requirements

- [Terraform](https://www.terraform.io/downloads.html) >= 1.0
- [Go](https://golang.org/doc/install) >= 1.21 (for building from source)
- RustGenie application running and accessible

## Building the Provider

1. Clone this repository
2. Run `make build` to build the provider
3. Run `make install` to install it locally

For Apple Silicon Macs, update the `OS_ARCH` in the Makefile to `darwin_arm64`.

## Using the Provider

### Provider Configuration

```hcl
terraform {
  required_providers {
    rustgenie = {
      source  = "hashicorp.com/edu/rustgenie"
      version = "0.1.0"
    }
  }
}

provider "rustgenie" {
  endpoint = "http://127.0.0.1:8000"  # Your RustGenie API endpoint
  # api_key  = "your-api-key"         # Optional: API key if authentication is required
}
```

### Resource: rustgenie_monitor

Creates and manages a monitor.

#### Example Usage

```hcl
resource "rustgenie_monitor" "api_health" {
  name           = "api-health-check"
  display_name   = "API Health Check"
  description    = "Monitors the health endpoint of our main API"
  url            = "https://api.example.com/health"
  monitor_type   = "http"
  check_interval = 60
  timeout        = 30
  is_active      = true
}
```

#### Argument Reference

- `name` - (Required) Unique identifier for the monitor
- `display_name` - (Required) Human-readable name for the monitor
- `description` - (Optional) Description of what the monitor checks
- `url` - (Optional) URL to monitor (required for HTTP and TCP monitors)
- `monitor_type` - (Optional) Type of monitor: `http`, `tcp`, `ping`, `dns`, `custom` (default: `http`)
- `check_interval` - (Optional) How often to check in seconds (default: `60`)
- `timeout` - (Optional) Timeout for each check in seconds (default: `30`)
- `is_active` - (Optional) Whether the monitor is active (default: `true`)

#### Attribute Reference

- `id` - The monitor ID assigned by the RustGenie application

### Data Source: rustgenie_monitor

Reads information about an existing monitor.

#### Example Usage

```hcl
data "rustgenie_monitor" "existing" {
  name = "api-health-check"
}

output "monitor_details" {
  value = {
    id             = data.rustgenie_monitor.existing.id
    display_name   = data.rustgenie_monitor.existing.display_name
    url            = data.rustgenie_monitor.existing.url
    check_interval = data.rustgenie_monitor.existing.check_interval
  }
}
```

#### Argument Reference

- `name` - (Required) The unique name of the monitor to look up

#### Attribute Reference

All monitor attributes are exported.

## Example: Complete Infrastructure

```hcl
# Configure the provider
provider "rustgenie" {
  endpoint = var.rustgenie_endpoint
}

# Create monitors for different services
resource "rustgenie_monitor" "web_app" {
  name           = "production-web-app"
  display_name   = "Production Web Application"
  description    = "Main customer-facing web application"
  url            = "https://app.example.com"
  monitor_type   = "http"
  check_interval = 30
  timeout        = 10
  is_active      = true
}

resource "rustgenie_monitor" "api" {
  name           = "production-api"
  display_name   = "Production API"
  description    = "REST API for mobile and web clients"
  url            = "https://api.example.com/health"
  monitor_type   = "http"
  check_interval = 30
  timeout        = 10
  is_active      = true
}

resource "rustgenie_monitor" "database" {
  name           = "production-database"
  display_name   = "Production Database"
  description    = "PostgreSQL database cluster"
  url            = "tcp://db.example.com:5432"
  monitor_type   = "tcp"
  check_interval = 60
  timeout        = 5
  is_active      = true
}

# Output monitor IDs for reference
output "monitor_ids" {
  value = {
    web_app  = rustgenie_monitor.web_app.id
    api      = rustgenie_monitor.api.id
    database = rustgenie_monitor.database.id
  }
}
```

## Development

### Running in Debug Mode

For development and debugging:

```bash
# Build with debug symbols
make dev

# Run in debug mode
go run main.go -debug

# Set the debug output in your Terraform configuration
export TF_REATTACH_PROVIDERS='{"hashicorp.com/edu/rustgenie":{"Protocol":"grpc","Pid":12345,"Test":true,"Addr":{"Network":"unix","String":"/tmp/plugin123456"}}}'
```

### Running Tests

```bash
make test
```

### Updating Dependencies

```bash
make deps
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License.