# Built from java-rust/, because the port reads the original JSP files and
# images from java/web at compile time.
FROM rust:1.90-slim AS build
WORKDIR /src
COPY java/web java/web
COPY rust/Cargo.toml rust/Cargo.lock rust/
COPY rust/src rust/src
COPY rust/templates rust/templates
RUN cd rust && cargo build --release --locked

FROM debian:bookworm-slim
COPY --from=build /src/rust/target/release/flyaway /usr/local/bin/flyaway
EXPOSE 8080
CMD ["flyaway"]
