FROM python:3.8.2-buster

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
            libprotobuf-dev \
            libsodium-dev \
            protobuf-compiler \
            software-properties-common; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

RUN git clone https://github.com/exonum/exonum-launcher && cd exonum-launcher \
    && pip3 install -r requirements.txt && pip3 install -e .

COPY cfg/crypto-election.yml .
COPY launcher/launch.sh .

CMD [ "./launch.sh" ]
