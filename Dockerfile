FROM rust:latest

WORKDIR /opt

COPY . .

RUN ["cargo", "build"]
