<div align="center">

# onigiri_server

device and user management API for **project onigiri**

</div>

## SETTING UP FOR DEVELOPMENT

Some features of the nightly build of `rustfmt` are required for
`rust-analyzer` and `clippy`, ensure that you have ran:
```
$ rustup install nightly
```

Next, install the git hook that will automatically format and lint your code on
commit with:
```
$ just devsetup
```
