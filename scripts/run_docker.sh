#!/bin/bash

# Check if image exists
if ! docker images -q my-rust-image > /dev/null 2>&1; then
    # Image does not exist, build it
    docker build -t my-rust-image .
    # Run container after building
    docker run -v /c/Users/liaoj/git/NeoRust:/workspace -v cargo-cache:/usr/local/cargo/registry -it my-rust-image
else
    # Image exists, directly run container
    docker run -v /c/Users/liaoj/git/NeoRust:/workspace -v cargo-cache:/usr/local/cargo/registry -it my-rust-image
fi