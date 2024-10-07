FROM rust

WORKDIR /usr/src/backup_db
COPY . .

RUN cargo install --path .

CMD ["backup_db"]
