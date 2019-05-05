# Nest-Server [![Build Status](https://travis-ci.org/raven-os/nest-server.svg?branch=master)](https://travis-ci.org/raven-os/nest-server)

An HTTP server hosting packages for Nest.

## Build dependencies

* rustup, with the latest nightly toolchain available

## Building Nest-Server

Compiling Nest-Server is pretty straightforward:

```bash
cargo build
```

## Running tests

If you want to be sure everything went correctly when compiling Nest-Server, you can run the tests:

```bash
cargo test
```

## Hosting

If you want to host some custom-made packages on your own Nest-Server instance, you may find the `Hosting.md` guide useful.

## API

Nest-Server provides an API to question the server about its content. This API is, among others, used by Nest when pulling
a repository or when downloading a package.

All the available API routes are listed in `Api.md`.
