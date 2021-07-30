# Base project

## Introduction

This is a base project all set up, using

* Rust with actix\_web and python with falcon as backends

* Elm and js as frontends (with snowpack for bundling)

* nginx for coordination

## Launching the code

The code can be launched by docker-compose. It has the backend and nginx to
serve statics.

It watches for changes for the backend with cargo wath and gunicorn --reload
but not for the frontend. If you want to watch the frontend, use snowpack.

To launch the docker-compose use

```bash

$ yarn build
$ docker-compose up

```

Before you launch the docker compose, you need to build the frontend if you
want to use the built frontend.

Snowpack can launch the dev server and has proxies to the docker-compose
backend.

To launch the frontend use

```bash

$ snowpack dev

```

## Testing

Unit tests are all set up for the four languages

The project can be used with Docker

to launch the tests of the **js** part, use

```bash

$ yarn mocha

```

to launch the tests of the **elm** part, use

```bash

$ elm-test

```

to launch the tests of the **python** part, use

```bash

$ pytest

```

to launch the tests of the **rust** part, use

```bash

$ cargo test

```
