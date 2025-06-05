#!/bin/bash

# Create a test incident
curl -X POST http://localhost:3000/api/incidents \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Database Connection Issues",
    "message": "We are experiencing intermittent database connection failures affecting some services. Our team is investigating the issue.",
    "severity": "warning",
    "affected_monitors": [1, 2, 3],
    "started_at": "'$(date -u +"%Y-%m-%dT%H:%M:%SZ")'"
  }'

echo -e "\n\nIncident created. Check the status page to see the incident banner."