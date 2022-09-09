FROM ekidd/rust-musl-builder AS build
WORKDIR /app/
COPY --chown=rust:rust . .
RUN cargo install --path .
FROM scratch
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/morpho-server /morpho-server
WORKDIR /app/
COPY ./static /app/static/
COPY ./cert.pem /app/cert.pem
COPY ./privkey.pem /app/privkey.pem
EXPOSE 443
CMD ["/morpho-server"]
