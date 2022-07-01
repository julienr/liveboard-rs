DROP DATABASE liveboard;
CREATE DATABASE liveboard OWNER postgres;
\c liveboard;

CREATE TABLE shapes (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    shape JSON NOT NULL
);