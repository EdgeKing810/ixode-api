FROM rust:slim-buster

RUN mkdir /app
WORKDIR /app

COPY . /app/

RUN rustc -V
RUN cargo --version

EXPOSE 8000
CMD ["cargo", "run", "--release"]
