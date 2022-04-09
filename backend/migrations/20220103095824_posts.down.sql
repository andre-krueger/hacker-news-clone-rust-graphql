-- Add down migration script here
DROP TRIGGER set_timestamp on posts;
DROP FUNCTION trigger_set_timestamp;
DROP TABLE posts;
