FROM rust:1.90-slim AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock* ./
COPY src src
RUN cargo build --release --locked

FROM debian:bookworm-slim
COPY --from=build /src/target/release/students /usr/local/bin/students
EXPOSE 8080
CMD ["students"]
