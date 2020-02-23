FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y software-properties-common git \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get install -y libsodium-dev protobuf-compiler \
        python3-pip python3-setuptools \
    && rm -rf /var/lib/apt/lists/* \
    && pip3 install exonum-launcher --upgrade --no-binary=protobuf

WORKDIR /usr/src

COPY cfg/crypto-election.yml .
COPY runtime/launch.sh .

CMD [ "launch.sh" ]
