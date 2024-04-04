FROM rust:latest

# Set the environment to non-interactive (to avoid tzdata prompting for geographic area)
ARG DEBIAN_FRONTEND=noninteractive

# Update the package list and install dependencies
RUN apt-get update \
    && apt-get install -y libssl-dev gcc git make wget \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Clone the desired repository (if necessary)
RUN git clone https://github.com/baidu/rust-sgx-sdk.git /rust-sgx-sdk

# Set environment variable for the Rust SGX SDK
ENV RUST_SGX_SDK=/rust-sgx-sdk

# Set your working directory
WORKDIR /workspace

# Install nightly toolchain and the necessary components
RUN rustup default nightly \
    && rustup update \
    && rustup component add cargo clippy rust-analyzer rust-src rustfmt rustc-dev \
    && rustup target add wasm32-unknown-unknown wasm32-wasi

# Clean up the cache of rustup downloads to reduce the image size
RUN rustup self clean
