CREATE TABLE IF NOT EXISTS todos (
    id          SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    done        INTEGER NOT NULL DEFAULT 0
);