CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    user_telegram_id BIGINT UNIQUE NOT NULL,
    api_key TEXT NOT NULL,
    encrypted_secret TEXT NOT NULL,
    exchange TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE IF NOT EXISTS strategies (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_name TEXT NOT NULL,
    enabled BOOLEAN DEFAULT false NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);
