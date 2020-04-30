# potatosync-rust
The potatosync api, but in rust with some added features for the v2 release

## How to install
#### Prerequisites
* Latest Rust Nightly
* Cargo installed

#### Steps

* Install Postgres client dependencies
```
apt install libpq
```
* Install Diesel CLI 
```
cargo install diesel_cli --no-default-features --features postgres
``` 
* Create tables
```
  diesel migration run
```
* Build and run
```
cargo run --debug for debug build, --release for release build (Faster)
```
