FROM scratch
COPY target/x86_64-unknown-linux-musl/release/morpho-server /morpho-server
WORKDIR /app/
COPY ./static /app/static/
EXPOSE 443
CMD ["/morpho-server"]
