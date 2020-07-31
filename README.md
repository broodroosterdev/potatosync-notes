# potatosync-notes
Api that handles syncing of the PotatoSync app, written in rust.

## Backstory
This api is going to replace the [previous sync api](https://github.com/ATechnoHazard/potatosync).

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
