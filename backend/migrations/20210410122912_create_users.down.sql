-- Add down migration script here
DROP TRIGGER set_timestamp on users;
DROP FUNCTION trigger_set_timestamp;
DROP TABLE user_roles;
DROP TABLE roles;
DROP TABLE users;
