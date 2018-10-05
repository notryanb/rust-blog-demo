## Rocket & Diesel Blog Demo

- Using rustc 1.31.0-nightly (8c4ad4e9e 2018-10-04)
- postgres backend
- This is a test app for [My Blog: lil bits](https://notryanb.github.io)

### Setup
- Must have `diesel_cli` installed. Directions can be found in the [Diesel Getting Started Guide](http://diesel.rs)
- Run `diesel migration run` to run all migrations on the postgres database
- Run `cargo run --bin seed` to seed the database with fake data. See `src/seed.rs` to setup default username/password for a login.
- Run `cargo run --bin main`, which will start the app.

### Notes (2018-10-05)
- Rocket `0.3.17` doesn't seem to compile currently, so I reverted to `0.3.11` to get it working. There happens to be a problem on Windows machines due to the [ring dependency and spectre vulnerability](https://github.com/SergioBenitez/Rocket/issues/779)
- I should make a CHANGELOG now that a few people have mentioned using this project as an example....
