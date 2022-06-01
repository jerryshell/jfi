FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/jfi_http_api .

CMD ["./jfi_http_api"]