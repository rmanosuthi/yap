FROM docker.io/library/rust:1.92.0-trixie

WORKDIR /app
COPY --parents Cargo.lock Cargo.toml client common server .
RUN cargo build --release -p client

VOLUME /deploy

CMD ./target/release/client
