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

RUN apt-get update --allow-insecure-repositories && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    ca-certificates \
    build-essential \
    make \
    perl \
    gcc \
    libc6-dev

# build mdbook-pdf
RUN cargo install --path .

# build other used backends and packages
RUN cargo install mdbook --vers ${MDBOOK_VERSION} --verbose
RUN cargo install mdbook-linkcheck --vers ${MDBOOK_LINKCHECK_VERSION} --verbose
RUN cargo install mdbook-mermaid --vers ${MDBOOK_MERMAID_VERSION} --verbose
RUN cargo install mdbook-toc --vers ${MDBOOK_TOC_VERSION} --verbose
RUN cargo install mdbook-plantuml --vers ${MDBOOK_PLANTUML_VERSION} --verbose
RUN cargo install mdbook-open-on-gh --vers ${MDBOOK_OPEN_ON_GH_VERSION} --verbose
RUN cargo install mdbook-graphviz --vers ${MDBOOK_GRAPHVIZ_VERSION} --verbose
RUN cargo install mdbook-katex --vers ${MDBOOK_KATEX_VERSION} --verbose

FROM pandoc/ubuntu-latex AS mdbook-pdf
COPY --from=builder /usr/local/cargo/bin/mdbook* /usr/bin/
RUN apt-get update --allow-insecure-repositories \
    && apt-get install --no-install-recommends -y \
    ca-certificates \
    texlive-xetex \
    graphviz \
    plantuml \
    && rm -rf /var/cache/apt/lists
SHELL ["/bin/bash"]
WORKDIR /book
ENTRYPOINT [ "/usr/bin/mdbook" ]