FROM rust
WORKDIR /usr/src/basics-web
COPY . .
RUN cargo build
CMD ["cargo", "run"]
