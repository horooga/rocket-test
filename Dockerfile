FROM rust:1-slim AS builder
WORKDIR /build
COPY . .
RUN --mount=type=cache,target=/build/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build; \
    cp target/debug/rocket-test main

FROM nginx
WORKDIR /app
RUN mkdir static templates
COPY --from=builder /build/main ./main
COPY --from=builder /build/Rocket.toml ./static
COPY --from=builder /build/static ./static
COPY --from=builder /build/templates ./templates
COPY --from=builder /build/wrapper_script ./wrapper_script
COPY --from=builder /build/nginx.conf /usr/nginx/nginx.conf
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
CMD sh ./wrapper_script
