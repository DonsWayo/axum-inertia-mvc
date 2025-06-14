-- Create sessions table for tower-sessions
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);

-- Create index for expiry_date to help with cleanup
CREATE INDEX IF NOT EXISTS idx_sessions_expiry ON sessions(expiry_date);