# Basics
FROM rust:1.74-alpine
WORKDIR /app

# Env
ENV TZ="America/Sao_Paulo"

# Update the container
RUN apk upgrade --no-cache --update
RUN apk add --no-cache bash libressl-dev musl-dev tzdata

# Configure the user
RUN adduser --disabled-password prod
RUN chown prod -R /app
USER prod

# Build the project
COPY --chown=prod ./src ./src
COPY --chown=prod ./Cargo.* ./
RUN cargo build --release

# Run the project
CMD cargo run --release
