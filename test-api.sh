#!/bin/bash

# Test script for Status Monitor API

echo "1. Creating a monitor..."
MONITOR_RESPONSE=$(curl -s -X POST http://localhost:8000/status/api/monitors \
  -H "Content-Type: application/json" \
  -d '{
    "name": "api-health",
    "display_name": "API Health Check",
    "description": "Monitors the health of our main API",
    "url": "https://safeguru.com",
    "monitor_type": "http",
    "check_interval": 60,
    "timeout": 30,
    "is_active": true,
    "metadata": null
  }')

echo "Response: $MONITOR_RESPONSE"
MONITOR_ID=$(echo $MONITOR_RESPONSE | sed -n 's/.*"id":\([0-9]*\).*/\1/p')
echo "Created monitor with ID: $MONITOR_ID"

echo -e "\n2. Creating another monitor..."
curl -s -X POST http://localhost:8000/status/api/monitors \
  -H "Content-Type: application/json" \
  -d '{
    "name": "database-health",
    "display_name": "Database Health",
    "description": "Monitors PostgreSQL database availability",
    "url": "postgres://localhost:5432",
    "monitor_type": "tcp",
    "check_interval": 30,
    "timeout": 10,
    "is_active": true,
    "metadata": {"port": 5432}
  }' | jq .

echo -e "\n3. Listing all monitors..."
curl -s http://localhost:8000/status/api/monitors | jq .

echo -e "\n4. Recording some status events for monitor $MONITOR_ID..."

# Add operational status
curl -s -X POST http://localhost:8000/status/api/monitors/$MONITOR_ID/events \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": '$MONITOR_ID',
    "status": "operational",
    "response_time": 125,
    "status_code": 200,
    "error_message": null,
    "metadata": {"region": "us-east-1"}
  }' | jq .

# Add degraded status
curl -s -X POST http://localhost:8000/status/api/monitors/$MONITOR_ID/events \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": '$MONITOR_ID',
    "status": "degraded",
    "response_time": 2500,
    "status_code": 200,
    "error_message": "Response time exceeded threshold",
    "metadata": {"region": "us-east-1"}
  }' | jq .

# Add partial outage
curl -s -X POST http://localhost:8000/status/api/monitors/$MONITOR_ID/events \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": '$MONITOR_ID',
    "status": "partial_outage",
    "response_time": null,
    "status_code": 503,
    "error_message": "Service temporarily unavailable",
    "metadata": {"region": "us-east-1"}
  }' | jq .

# Back to operational
curl -s -X POST http://localhost:8000/status/api/monitors/$MONITOR_ID/events \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": '$MONITOR_ID',
    "status": "operational",
    "response_time": 98,
    "status_code": 200,
    "error_message": null,
    "metadata": {"region": "us-east-1"}
  }' | jq .

echo -e "\n5. Getting monitor details..."
curl -s http://localhost:8000/status/api/monitors/$MONITOR_ID | jq .

echo -e "\n6. Updating monitor..."
curl -s -X PUT http://localhost:8000/status/api/monitors/$MONITOR_ID \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "Main API Health Check (Updated)",
    "check_interval": 120
  }' | jq .

echo -e "\n7. Check the status page at: http://localhost:8000/status"