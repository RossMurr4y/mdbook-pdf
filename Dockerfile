FROM rust:1.46.0-slim AS builder

ARG MDBOOK_VERSION="0.4.7"
ARG MDBOOK_LINKCHECK_VERSION="0.7.4"
ARG MDBOOK_MERMAID_VERSION="0.8.0"
ARG MDBOOK_TOC_VERSION="0.6.1"
ARG MDBOOK_PLANTUML_VERSION="0.7.0"
ARG MDBOOK_OPEN_ON_GH_VERSION="2.0.0"
ARG MDBOOK_GRAPHVIZ_VERSION="0.0.2"
ARG MDBOOK_KATEX_VERSION="0.2.8"

COPY . ./

RUN apt-get update --allow-insecure-repositories; \
    apt-get install -y \
        libssl-dev \
        pkg-config \
        ca-certificates \
        build-essential \
        make \
        perl \
        gcc \
        libc6-dev; \
    dpkgArch="$(dpkg --print-architecture)"; \
    echo "Arch: ${dpkgArch}"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        i386) rustArch='i686-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    echo "Setting cargo default arch to: ${rustArch}"; \
    rustup set default-host ${rustArch}; \
    cargo install --path . ; \
    cargo install mdbook --vers ${MDBOOK_VERSION} --verbose; \
    cargo install mdbook-linkcheck --vers ${MDBOOK_LINKCHECK_VERSION} --verbose; \
    cargo install mdbook-mermaid --vers ${MDBOOK_MERMAID_VERSION} --verbose; \
    cargo install mdbook-toc --vers ${MDBOOK_TOC_VERSION} --verbose; \
    cargo install mdbook-plantuml --vers ${MDBOOK_PLANTUML_VERSION} --verbose; \
    cargo install mdbook-open-on-gh --vers ${MDBOOK_OPEN_ON_GH_VERSION} --verbose; \
    cargo install mdbook-graphviz --vers ${MDBOOK_GRAPHVIZ_VERSION} --verbose; \
    cargo install mdbook-katex --vers ${MDBOOK_KATEX_VERSION} --verbose;

FROM pandoc/ubuntu-latex AS mdbook-pdf
COPY --from=builder /usr/local/cargo/bin/mdbook* /usr/bin/
RUN apt-get update --allow-insecure-repositories; \
    apt-get install --no-install-recommends -y \
        ca-certificates \
        texlive-xetex \
        graphviz \
        plantuml \
        && rm -rf /var/cache/apt/lists
SHELL ["/bin/bash"]
WORKDIR /book
ENTRYPOINT [ "/usr/bin/mdbook" ]