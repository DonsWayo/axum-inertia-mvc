-- Create monitors table
CREATE TABLE IF NOT EXISTS monitors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    url VARCHAR(500),
    monitor_type VARCHAR(50) NOT NULL DEFAULT 'http',
    check_interval INTEGER NOT NULL DEFAULT 60, -- in seconds
    timeout INTEGER NOT NULL DEFAULT 30, -- in seconds
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_monitors_name ON monitors(name);
CREATE INDEX idx_monitors_is_active ON monitors(is_active);
CREATE INDEX idx_monitors_monitor_type ON monitors(monitor_type);

-- Add updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_monitors_updated_at BEFORE UPDATE
    ON monitors FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();