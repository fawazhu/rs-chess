FROM rust:1.71-bookworm as build
WORKDIR /work
COPY src ./src
COPY Cargo.lock .
COPY Cargo.toml .
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target 'x86_64-unknown-linux-gnu'

FROM scratch
COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/chess /chess
CMD [ "/chess" ]
