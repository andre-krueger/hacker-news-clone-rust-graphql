CREATE OR REPLACE FUNCTION trigger_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL NOT NULL,
    username   TEXT        NOT NULL,
    password   TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    UNIQUE (username)
);

CREATE TYPE role AS ENUM ('admin', 'user');

CREATE TABLE IF NOT EXISTS roles
(
    id        SERIAL NOT NULL,
    role_name role NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (role_name)
);

CREATE TABLE IF NOT EXISTS user_roles
(
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id),
    CONSTRAINT fk_role FOREIGN KEY (role_id) REFERENCES roles (id)
);

INSERT INTO roles (role_name)
VALUES ('admin');
INSERT INTO roles (role_name)
VALUES ('user');

CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
