-- Create incidents table for status page announcements
CREATE TABLE IF NOT EXISTS incidents (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('info', 'warning', 'critical')),
    affected_monitors INTEGER[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    is_resolved BOOLEAN NOT NULL DEFAULT FALSE,
    metadata JSONB DEFAULT '{}'
);

-- Create index for active incidents
CREATE INDEX idx_incidents_active ON incidents(is_resolved, started_at DESC) 
WHERE is_resolved = FALSE;

-- Create index for affected monitors
CREATE INDEX idx_incidents_affected_monitors ON incidents USING GIN(affected_monitors);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_incidents_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_incidents_updated_at_trigger
    BEFORE UPDATE ON incidents
    FOR EACH ROW
    EXECUTE FUNCTION update_incidents_updated_at();