FROM python:3.8.2-buster

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
            libprotobuf-dev \
            libsodium-dev \
            protobuf-compiler \
            software-properties-common; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

# Up to release 0.2.0: https://github.com/exonum/exonum-launcher/releases/tag/v0.2.0
RUN git clone -n https://github.com/exonum/exonum-launcher && cd exonum-launcher \
    && git checkout 0cdd505bd0bf717f618012689293f61d9b675bf0 \
    && pip3 install -r requirements.txt && pip3 install -e .

COPY cfg/crypto-election.yml .
COPY launcher/launch.sh .

CMD [ "./launch.sh" ]
