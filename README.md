To check for outdated dependencies for the backend, run `cargo outdated`.

To update the outdated dependencies, run `cargo upgrade`. Aftewards, the dependencies need to be pinned again until [this](https://github.com/killercup/cargo-edit/issues/454) is resolved.

To update the frontend dependencies, run `npm run update-all`.

To nuke and recreate the database, run `sqlx database drop -y && sqlx database create`

To recompile and restart the server, run `cargo watch -x "run"`.

To create an admin user, run `cargo run --bin create_admin USERNAME PASSWORD`.

To regenerate the schema on any backend change, run `cargo watch -s "(cd ../frontend; npm run download-schema)"`
