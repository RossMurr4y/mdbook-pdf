# mdbook-pdf

A PDF backend for [mdBook](https://rust-lang.github.io/mdBook/)

## Usage

Update your `book.toml` with the pdf backend.

If you've only had the default html backend until now, make sure you specify it now so that both backends are run.

```toml
[book]
title = "Example Book"
authors = ["Your Name"]

[output.html]

[output.pdf]
```

Build your book to output your PDF alongside the other backend outputs in `book/pdf`

```terminal
mdbook build
```

## Configuration

Just specifying the PDF output type is sufficient configuration.

The below example shows the full configuration options with defaults.

```toml
...

[output.pdf]

[output.pdf.pandoc]
output_name = "<Name of Book>"
engine = "xelatex"
main_font = "Liberation Serif"
```

### output.pdf

#### output_name

The name of the output PDF document. Defaults to the name of your book when not provided.

### output.pdf.pandoc
#### engine

The PDF engine to use to perform the translation to PDF. Default is to use `xelatex` which must be installed (available within the docker container).

#### main_font

This is the primary font used by the pdf-engine.

Available for the following engines:

- pdflatex
- lualatex
- xelatex
- context

Unlike when using Pandoc directly, this setting is used for all of the pdf-engines for which it is available. This consolidation of configuration was done so as to make the module as easy to pick up and use as possible without needing to understand the inner-workings of Pandoc and latex.

## Installation

> `mdbook-pdf` is currently in heavy development and is not yet available with cargo.
> Recommended use for now is with [Docker](#docker).

- install [XeLatex](http://xetex.sourceforge.net) (for usage with XeLatex - default)
- install [TeXLive](https://www.tug.org/texlive/) (for usage with pdflatex)
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

Builds should be performed with `buildx` to ensure compatability with your architecture.

```terminal
docker buildx build .
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
