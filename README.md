# mdbook-pdf

A PDF backend for [mdBook](https://rust-lang.github.io/mdBook/)

## Usage

- install TeXLive
- install pandoc

Install `mdbook-pdf` binary

```bash
cargo install mdbook-pdf
```

Add the following to your `book.toml` to enable the PDF backend.

The default settings are provided below, with their usage listed after.

```bash
[output.pdf]
```