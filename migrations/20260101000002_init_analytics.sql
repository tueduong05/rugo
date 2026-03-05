CREATE TABLE IF NOT EXISTS link_analytics (
    id BIGSERIAL PRIMARY KEY,
    link_id BIGINT NOT NULL REFERENCES links(id) ON DELETE CASCADE,
    referrer TEXT,
    user_agent TEXT,
    ua_info JSONB NOT NULL,
    geo JSONB NOT NULL,
    masked_ip VARCHAR(45) NOT NULL,
    clicked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_link_analytics_link_id ON link_analytics(link_id);
CREATE INDEX IF NOT EXISTS idx_link_analytics_clicked_at ON link_analytics(clicked_at);

CREATE INDEX IF NOT EXISTS idx_link_analytics_ua_info ON link_analytics USING GIN (ua_info);
CREATE INDEX IF NOT EXISTS idx_link_analytics_geo ON link_analytics USING GIN (geo);
