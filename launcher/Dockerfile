FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y software-properties-common apt-utils git \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get install -y libsodium-dev protobuf-compiler \
        python3-pip python3-setuptools \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

RUN git clone https://github.com/exonum/exonum-launcher && cd exonum-launcher \
    && pip3 install -r requirements.txt && pip3 install -e .

COPY cfg/crypto-election.yml .
COPY launcher/launch.sh .

CMD [ "./launch.sh" ]
