CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    public_key TEXT UNIQUE NOT NULL,
    alias TEXT NOT NULL,
    capacity_sats BIGINT NOT NULL,
    capacity_btc DECIMAL(20,8) NOT NULL,
    first_seen_unix BIGINT NOT NULL,
    first_seen_formatted TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);