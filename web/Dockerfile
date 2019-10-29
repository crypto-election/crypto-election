FROM rust:1.31

WORKDIR /usr/src/crypto_election_web
COPY . .

RUN cargo install --path .

CMD ["crypto_election_web"]

EXPOSE 80 443
