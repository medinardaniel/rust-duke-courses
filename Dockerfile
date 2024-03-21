FROM rust:latest as builder
ENV APP rust_duke_courses
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/rust_duke_courses
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
COPY class_data.json /usr/src/rust_duke_courses/class_data.json
EXPOSE 8080
CMD ["rust_duke_courses"]
