FROM rust

EXPOSE 80

# Install build tools
RUN rustup default nightly
RUN wget -qO- https://deb.nodesource.com/setup_10.x | bash -
RUN apt-get install -y -q nodejs

# Copy app and set working directory
COPY . /app
WORKDIR /app

# Compile backend
RUN cargo build --release

# Compile frontend
RUN cd front && npm install && npm run build

# Setup environnement and run
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT=80
CMD npm run build ; cargo run