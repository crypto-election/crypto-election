FROM ubuntu:18.04

ENV ROCKSDB_LIB_DIR=/usr/lib
ENV SNAPPY_LIB_DIR=/usr/lib/x86_64-linux-gnu

RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get update \
    && apt-get install -y curl git \
    build-essential libsodium-dev libsnappy-dev \
    librocksdb-dev pkg-config clang-7 lldb-7 lld-7 \
    libprotobuf-dev protobuf-compiler

ENV RUSTUP_HOME=/usr
ENV CARGO_HOME=/usr

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable

WORKDIR /usr/src
COPY . /dev/null
RUN cd node && cargo update && cargo install --path .

ENV COMMAND=run-async
ENV SHARED_DIR=/usr/public
ENV LABEL=node
ENV LABELS node1 node2
ENV VALIDATORS_ARG=""

CMD \
    crypto-election-node $COMMAND \
        --public-path $SHARED_DIR \
        #--attempt-number  \
        #--attempt-delay  \
        --label $LABEL \
        --labels $LABELS \
        --peer-address "${awk 'END{print $1}' /etc/hosts}" \
        #--listen-address  \
        --no-password \
        #--master-key-pass  \
        #--master-key-path  \
        #--output-dir  \
        --final-config-path node.toml \
        --public-api-address 0.0.0.0:8000 \
        --private-api-address 0.0.0.0:6330 \
        #--public-allow-origin  \
        #--private-allow-origin  \
        --db-path db \
        $VALIDATORS_ARG
