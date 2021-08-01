FROM rossmurr4y/mdbook-pdf as bins

FROM rust:1.54-buster as busterdev

COPY --from=bins /usr/bin/mdbook* /usr/bin

RUN apt-get update --allow-insecure-repositories && \
    apt-get install -y \
    openjdk-11-jre-headless:amd64 \
    libssl-dev \
    pkg-config \
    ca-certificates \
    build-essential \
    make \
    perl \
    gcc \
    libc6-dev \
    graphviz \
    plantuml \
    pandoc \
    texlive \
    texlive-xetex

RUN cargo install starship
RUN echo 'eval "$(starship init bash)"' >> ~/.bashrc

SHELL ["/bin/bash"]
CMD [ "/bin/bash" ]