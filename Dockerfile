FROM rust:latest as builder
ENV APP rust_duke_courses
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rust_duke_courses/target/release/rust_duke_courses /usr/local/bin/
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["rust_duke_courses"]