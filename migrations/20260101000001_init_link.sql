CREATE TABLE IF NOT EXISTS links (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL, 
    original_link TEXT NOT NULL,
    short_code VARCHAR(20) UNIQUE NOT NULL,
    is_custom BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMPTZ,
    hashed_password TEXT,
    max_clicks INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() 
);

CREATE INDEX IF NOT EXISTS idx_links_user_id ON links(user_id);
