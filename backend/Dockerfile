##################
### BASE STAGE ###
##################
FROM rust:1.54 as base

WORKDIR /app

# Install required tools
RUN cargo install strip_cargo_version sqlx-cli

# Install build-target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Required by `brotli-sys`
RUN apt-get update && apt-get install -y musl-tools

###########################
### STRIP-VERSION STAGE ###
###########################
FROM base as strip-version

# Strip version from Cargo.*
# This avoids cache invalidations (rebuilding all deps) when bumping the version number
COPY Cargo.toml Cargo.lock ./
RUN strip_cargo_version

###################
### BUILD STAGE ###
###################
FROM base as build

# Create a dummy binary for pre-compiling dependencies (for caching)
RUN cargo init .
COPY --from=strip-version /app/Cargo.* ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the actual source files
COPY . .

# Compile the final binary
RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/backend

########################
### PRODUCTION STAGE ###
########################
FROM scratch
WORKDIR /

ENV PORT=8080
EXPOSE 8080

COPY migrations ./
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/backend ./hubbit

CMD ["./hubbit"]