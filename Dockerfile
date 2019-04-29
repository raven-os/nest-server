FROM rust

EXPOSE 80

# Install Rust nightly
RUN rustup default nightly

# Copy app and set working directory
COPY . /app
WORKDIR /app

# Compile backend
RUN cargo build --release

# Volumes to override
VOLUME /app/packages/
VOLUME /app/cache/

# Run
CMD cargo run --release
