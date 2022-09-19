# rust postgres dbadmin

Managing DB users across multiple Databases

> This is my first Rust script, so please be gentle (⌒‿⌒)

Reading through your local pgpass file, allows you to perform the following tasks:
- [C] Create user
- [D] Drop user
- [P] Change password
- [R] Review user
- [W] Alter user add Write permission
- [S] Search user;

You can bypass the menu passing argume directly to the command:

```bash
./rustdbadmin username action {{C|D|P|R|W|S}}
```

**Note: a password will be randomly generated and assigned to the account.**

## Getting started

Install Rust on your local machine, to do so please follow the official documentation

[Rust get starteds](https://www.rust-lang.org/learn/get-started)


### Get a local copy using git

```bash
git clone https://github.com/lunarust/rust_postgres_dbadmin.git
```

## Build or Run this project:

build your project with cargo build, run the following in your local repository directory:
```bash
// Run you project:
cargo run
//test your project with
cargo test
//build documentation for your project with
cargo doc --open --no-deps
//publish a library to crates.io with
cargo publish
// build a release version with
cargo build --release
```

You should be able to compile and build this code on either MacOS, Lin or Windows.
However, all tests have been performed only for Linux.

Build and documentation will be created under ./target/

## Todo
- [x] Create user
- [x] Drop user
- [x] Drop user with schema permission
- [ ] Drop user no prompt, but should we?
- [x] Add user to role (ie. write), not tested yet
- [x] Review user, checks permissions and roles
- [x] Change password, not tested yet
- [x] Search user with pattern


## Contributing


## Related projects

bits & pieces from [notryanb](https://github.com/notryanb/psql_connect/blob/master/src/main.rs)

## Licensing


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)