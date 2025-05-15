ARG TARGET_ARCH=linux/amd64

FROM rust:1.87.0 AS builder

WORKDIR /app

# Copy the Rust project files
COPY src /app/src
COPY rsc /app/rsc
COPY Cargo.lock /app
COPY Cargo.toml /app

ARG TARGET_ARCH
ADD docker/build-rust.sh /root
RUN bash /root/build-rust.sh $TARGET_ARCH

FROM --platform=$TARGET_ARCH debian

# Install libssl3
RUN apt update && apt install -y libssl3 chromium && rm -rf /var/lib/apt/lists/*


# Copy the built binary from the previous stage
COPY --from=builder --chmod=0755 /app/target/byos_rs /usr/local/bin/byos_rs

WORKDIR /app
CMD /usr/local/bin/byos_rs