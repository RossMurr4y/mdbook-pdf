FROM rossmurr4y/mdbook-pdf-bin AS bin
FROM rossmurr4y/mdbook
COPY --from=bin /usr/local/cargo/bin/mdbook-pdf /usr/bin/
RUN cargo install starship; \
    echo 'eval "$(starship init bash)"' >> ~/.bashrc
SHELL ["/bin/bash"]
CMD [ "/bin/bash" ]