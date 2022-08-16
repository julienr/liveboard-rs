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

CREATE TABLE boards (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    name TEXT NOT NULL
);

CREATE TABLE shapes (
    id SERIAL PRIMARY KEY,
    board_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    shape TEXT NOT NULL,
    CONSTRAINT fk_board
        FOREIGN KEY(board_id) REFERENCES boards(id)
);