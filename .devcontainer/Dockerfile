FROM ubuntu:24.04

RUN apt-get update && apt-get install -y curl git build-essential

# Install Node.js (globally)
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash -
RUN apt-get install -y nodejs

# Install pnpm (globally)
RUN npm install -g pnpm@10.19.0

USER ubuntu

# Install Rust (for user)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
