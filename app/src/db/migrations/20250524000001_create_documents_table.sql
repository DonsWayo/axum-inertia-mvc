CREATE TABLE IF NOT EXISTS documents (
    id SERIAL PRIMARY KEY,
    header VARCHAR(255) NOT NULL,
    type_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    target VARCHAR(50) NOT NULL,
    limit_value VARCHAR(50) NOT NULL,
    reviewer VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_documents_status ON documents(status);
CREATE INDEX IF NOT EXISTS idx_documents_type_name ON documents(type_name);
CREATE INDEX IF NOT EXISTS idx_documents_reviewer ON documents(reviewer);
