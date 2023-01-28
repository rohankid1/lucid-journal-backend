FROM rust:1.67.0-buster

COPY ./target/release/dream-journal.exe /opt/dream-journal.exe

RUN /opt/dream-journal.exe