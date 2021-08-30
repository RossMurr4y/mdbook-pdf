FROM rossmurr4y/rust-builder-base
COPY . ./
RUN cargo install --path .