# potatosync-rust
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fbroodroosterdev%2Fpotatosync-rust.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fbroodroosterdev%2Fpotatosync-rust?ref=badge_shield)

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


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fbroodroosterdev%2Fpotatosync-rust.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fbroodroosterdev%2Fpotatosync-rust?ref=badge_large)