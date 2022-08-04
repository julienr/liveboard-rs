/* Kill all other connections */
SELECT
	pg_terminate_backend(pg_stat_activity.pid)
FROM
	pg_stat_activity
WHERE
	pg_stat_activity.datname = 'database_name'
	AND pid <> pg_backend_pid();


/* Drop and re-create database */
DROP DATABASE liveboard;
CREATE DATABASE liveboard OWNER postgres;
\c liveboard;

CREATE TABLE shapes (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    shape JSON NOT NULL
);

CREATE TABLE boards (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    name TEXT NOT NULL
)