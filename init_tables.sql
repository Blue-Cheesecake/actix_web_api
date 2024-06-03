DROP TABLE IF EXISTS products;

CREATE TABLE
  products (
    id SERIAL PRIMARY KEY,
    name TEXT,
    status VARCHAR(100) NOT NULL CHECK (status IN ('draft', 'processing', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
  );