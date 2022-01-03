-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts
(
    id         SERIAL NOT NULL,
    text   TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
    );

CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON posts
    FOR EACH ROW
    EXECUTE PROCEDURE trigger_set_timestamp();
