-- Create status_events table
CREATE TABLE IF NOT EXISTS status_events (
    time TIMESTAMPTZ NOT NULL,
    monitor_id INTEGER NOT NULL REFERENCES monitors(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL, -- operational, degraded, partial_outage, major_outage, maintenance, unknown
    response_time INTEGER, -- in milliseconds
    status_code INTEGER,
    error_message TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('status_events', 'time');

-- Create indexes
CREATE INDEX idx_status_events_monitor_id_time ON status_events(monitor_id, time DESC);
CREATE INDEX idx_status_events_status ON status_events(status, time DESC);

-- Create composite index for efficient queries
CREATE INDEX idx_status_events_monitor_status_time ON status_events(monitor_id, status, time DESC);