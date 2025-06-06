# Terraform Provider for Status Monitor

This Terraform provider allows you to manage monitors in the Status Monitor application using Infrastructure as Code.

## Features

- ✅ Create, Read, Update, and Delete monitors
- ✅ Support for different monitor types (HTTP, TCP, etc.)
- ✅ Data source to read existing monitors
- ✅ Configurable endpoint and optional API key authentication

## Requirements

- [Terraform](https://www.terraform.io/downloads.html) >= 1.0
- [Go](https://golang.org/doc/install) >= 1.21 (for building from source)
- Status Monitor application running and accessible

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
    statusmonitor = {
      source  = "hashicorp.com/edu/statusmonitor"
      version = "0.1.0"
    }
  }
}

provider "statusmonitor" {
  endpoint = "http://127.0.0.1:8000"  # Your Status Monitor API endpoint
  # api_key  = "your-api-key"         # Optional: API key if authentication is required
}
```

### Resource: statusmonitor_monitor

Creates and manages a monitor.

#### Example Usage

```hcl
resource "statusmonitor_monitor" "api_health" {
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

- `id` - The monitor ID assigned by the Status Monitor application

### Data Source: statusmonitor_monitor

Reads information about an existing monitor.

#### Example Usage

```hcl
data "statusmonitor_monitor" "existing" {
  name = "api-health-check"
}

output "monitor_details" {
  value = {
    id             = data.statusmonitor_monitor.existing.id
    display_name   = data.statusmonitor_monitor.existing.display_name
    url            = data.statusmonitor_monitor.existing.url
    check_interval = data.statusmonitor_monitor.existing.check_interval
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
provider "statusmonitor" {
  endpoint = var.status_monitor_endpoint
}

# Create monitors for different services
resource "statusmonitor_monitor" "web_app" {
  name           = "production-web-app"
  display_name   = "Production Web Application"
  description    = "Main customer-facing web application"
  url            = "https://app.example.com"
  monitor_type   = "http"
  check_interval = 30
  timeout        = 10
  is_active      = true
}

resource "statusmonitor_monitor" "api" {
  name           = "production-api"
  display_name   = "Production API"
  description    = "REST API for mobile and web clients"
  url            = "https://api.example.com/health"
  monitor_type   = "http"
  check_interval = 30
  timeout        = 10
  is_active      = true
}

resource "statusmonitor_monitor" "database" {
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
    web_app  = statusmonitor_monitor.web_app.id
    api      = statusmonitor_monitor.api.id
    database = statusmonitor_monitor.database.id
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
export TF_REATTACH_PROVIDERS='{"hashicorp.com/edu/statusmonitor":{"Protocol":"grpc","Pid":12345,"Test":true,"Addr":{"Network":"unix","String":"/tmp/plugin123456"}}}'
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