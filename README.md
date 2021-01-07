# Base project

This is a base project all set up, using

* Rust with actix\_web and python with falcon as backends

* Elm and js as frontends (with snowpack for bundling)

* nginx for coordination

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
