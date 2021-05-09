FROM rust:latest as build
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/gcd-web-server /
EXPOSE 8080
CMD ["./gcd-web-server"]