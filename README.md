# Rust HTTP server to AWS EC2

This repository is a self-training exercise for running simple Rust HTTP server (with basic auth) in EC2 using Docker.

License "as applicable" - clearly borrowed scripts and code owned and licensed by the applicable party or author.

## Usage

  - Create Ubuntu EC2 instance in AWS
  - SSH to the instance
  - Clone and `cd` this repository
  - Build (with sudo if necessary)
    - `docker build -t rust-alpine -f ./Dockerfile .`
  - Start in background (with sudo if necessary)
    - `docker run -d -p 80:80 rust-alpine:latest`
  - Try accessing EC2's IP address with a browser which should prompt you basic auth credentials as hard-coded in `main.rs`
