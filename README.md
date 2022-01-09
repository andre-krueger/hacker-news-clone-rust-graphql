To check for outdated dependencies for the backend, run `cargo outdated`.

To update the outdated dependencies, run `cargo upgrade`. Aftewards, the dependencies need to be pinned again until [this](https://github.com/killercup/cargo-edit/issues/454) is resolved.

To update the frontend dependencies, run `npm run update-all`.

To nuke and recreate the database, run `sqlx database drop -y && sqlx database create`

To recompile and restart the server, run `cargo watch -x "run"`.

To create an admin user, run `cargo run --bin create_admin USERNAME PASSWORD`.

To regenerate the schema on any backend change, run `cargo watch -s "(cd ../frontend; yarn download-schema)"`

Before running, be sure to reverse both 8000 and 8081 via adb reverse.

### TODO
- [ ] Check progress on this ticket https://github.com/supermacro/neverthrow/issues/212


npx parcel build src/templates/index.html src/templates/guestbook.html 


./cg_clif/build/cargo-clif watch -c -x "run --bin backend" -w ../frontend_parcel/static -w .

to run nginx, redis and postgres, run this in project root
DEV=true nixos-shell configuration.nix
the website is then available under localhost:8080


watchexec -e tsx,html,css npm run parcel:build-unoptimized

install https://addons.mozilla.org/en-US/firefox/addon/live-reload/
with host url
https://localhost:8081/*
with source file urls
https://localhost:8081/static/*.js
https://localhost:8081/static/*.*.css
https://localhost:8081/*
