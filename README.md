# mdbook-pdf

A PDF backend for [mdBook](https://rust-lang.github.io/mdBook/)

## Usage

- install [TeXLive](https://www.tug.org/texlive/)
- install [pandoc](https://pandoc.org/installing.html)

Install `mdbook-pdf` binary from local clone

```bash
cargo install --path .
```

Add the following to your `book.toml` to enable the PDF backend.

The default settings are provided below, with their usage listed after.

```bash
[output.pdf]
```

## Docker

A minimal docker image is published containing all the prerequisites, `mdbook`, `mdbook-pdf` and most of the common `mdbook` backends.

```terminal
docker pull rossmurr4y/mdbook-pdf
```

### Build

```terminal
docker build .
```

### Tag

```terminal
docker image tag <tag> rossmurr4y/mdbook-pdf:latest
```

### Publish

```terminal
docker push rossmurr4y/mdbook-pdf
```

### docker-compose

The following `docker-compose.yml` example simplifies building/developing with `mdbook` & `mdbook-pdf`:

```yml
version: '3'

services:
  mdbook:
    container_name: mdbook
    image: rossmurr4y/mdbook-pdf:latest
    stdin_open: true
    tty: true
    ports:
      - 3000:3000
      - 3001:3001
    volumes:
      - ${PWD}/docs:/book
    command:
      - serve
      - --hostname
      - '0.0.0.0'
```
