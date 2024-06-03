-- start the docker-compose and use Adminer to execute these SQLs 
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    description VARCHAR,
    name VARCHAR NOT NULL,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    price DECIMAL NOT NULL,
    added_on DATE NOT NULL DEFAULT CURRENT_DATE,
    category_id INTEGER REFERENCES categories(id) ON DELETE NO ACTION,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_items_category_id ON items(category_id);

CREATE TABLE IF NOT EXISTS item_tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS items_taggings (
    item_id INTEGER REFERENCES items(id) NOT NULL,
    item_tag_id INTEGER REFERENCES item_tags(id) NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_items_name ON items(name);

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    customer_number SERIAL,
    items JSONB,
    ordered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    state VARCHAR NOT NULL DEFAULT 'created',
    inserted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);