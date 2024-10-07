FROM rust

RUN useradd -u 1010 -m backup_user

WORKDIR /usr/src/backup_db
COPY . .

RUN cargo install --path .

CMD ["backup_db"]
