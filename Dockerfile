FROM rust:slim-buster

RUN mkdir /app
WORKDIR /app

COPY . /app/

RUN apt update && \
    apt install -y pkg-config openssl

RUN rustc -V
RUN cargo --version

EXPOSE 8000
CMD ["cargo", "run", "--release"]
