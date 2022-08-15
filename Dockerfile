FROM rust:slim-buster

RUN mkdir /app
WORKDIR /app

COPY . /app/

RUN apt update && \
    apt install -y pkg-config openssl libssl-dev

RUN rustc -V
RUN cargo --version
RUN mkdir -p /db/data

RUN cargo fetch
RUN cargo build --release

EXPOSE 8080
CMD ["cargo", "run", "--release"]
