<div align="center">

# onigiri_server

device and user management API for **project onigiri**

[![book](https://img.shields.io/badge/book-website-orange)](https://karatsubalabs.github.io/onigiri-server/)
[![build](https://github.com/KaratsubaLabs/onigiri-server/workflows/Deploy/badge.svg)](https://github.com/KaratsubaLabs/onigiri-server/actions)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

## SETTING UP FOR DEVELOPMENT

Some features of the nightly build of `rustfmt` are required for
`rust-analyzer` and `clippy`, ensure that you have ran:
```
$ rustup install nightly
```

There are some other tools that should be installed to make development easier:
```
$ cargo install cargo-watch
```

Next, install the git hook that will automatically format and lint your code on
commit with:
```
$ just devsetup
```

## DATABASE SETUP

An instance of surrealdb needs to be up. You can run it as a docker container
```
$ docker run --rm -p 8000:8000 surrealdb/surrealdb:latest start --log debug --user <USERNAME> --pass <PASSWORD> memory
```

## READING DOCUMENTATION

Documentation on **project onigiri** can be read locally with the command
```
$ just book
```
or read online [here](https://karatsubalabs.github.io/onigiri-server/index.html).
