@echo off

// Check if image exists
docker images -q my-rust-image > nul 2>&1
if %ERRORLEVEL% NEQ 0 goto build

// Image exists, run container
docker run -v /c/Users/liaoj/git/NeoRust:/workspace -v cargo-cache:/usr/local/cargo/registry -it my-rust-image
goto end

:build
// Image does not exist, build it
docker build -t my-rust-image .
// Run container after building
docker run -v /c/Users/liaoj/git/NeoRust:/workspace -v cargo-cache:/usr/local/cargo/registry -it my-rust-image

:end