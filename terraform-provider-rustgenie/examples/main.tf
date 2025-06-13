terraform {
  required_providers {
    statusmonitor = {
      source = "hashicorp.com/edu/statusmonitor"
      version = "0.1.0"
    }
  }
}

provider "statusmonitor" {
  endpoint = "http://127.0.0.1:8000"
  # api_key  = "your-api-key" # Optional if API requires authentication
}

# Create a new monitor
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

# Create another monitor for database
resource "statusmonitor_monitor" "database" {
  name           = "database-connection"
  display_name   = "Database Connection Monitor"
  description    = "Monitors PostgreSQL database connectivity"
  url            = "tcp://db.example.com:5432"
  monitor_type   = "tcp"
  check_interval = 30
  timeout        = 10
  is_active      = true
}

# Data source to read existing monitor
data "statusmonitor_monitor" "existing" {
  name = "crm-api"
}

# Output the existing monitor details
output "existing_monitor_id" {
  value = data.statusmonitor_monitor.existing.id
}

output "existing_monitor_url" {
  value = data.statusmonitor_monitor.existing.url
}