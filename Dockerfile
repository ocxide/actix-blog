FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef --locked
WORKDIR /actix-blog

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path=recipe.json

FROM chef AS builder
COPY --from=planner /actix-blog/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target=x86_64-unknown-linux-musl --recipe-path=recipe.json

COPY . .
COPY --from=planner /actix-blog/migrations ./migrations

RUN cargo install wasm-bindgen-cli

RUN cargo build --release --target=x86_64-unknown-linux-musl

# Add mannually because actions-rs/toolchain@v1 doesn't support multiple targets
# Ussually, this install is cheap
RUN rustup target add wasm32-unknown-unknown
RUN chmod +x ./scripts/build-crs.sh
RUN ./scripts/build-crs.sh

FROM scratch
COPY --from=builder /actix-blog/target/x86_64-unknown-linux-musl/release/actix-blog /actix-blog
COPY --from=builder /actix-blog/pkg /pkg

ENTRYPOINT ["/actix-blog"]
EXPOSE 3000
