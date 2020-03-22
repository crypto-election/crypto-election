FROM ubuntu:18.04

ENV ROCKSDB_LIB_DIR=/usr/lib
ENV SNAPPY_LIB_DIR=/usr/lib/x86_64-linux-gnu

RUN apt-get update \
    && apt-get install -y software-properties-common apt-utils \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get update \
    && apt-get install -y curl git \
    build-essential libsodium-dev libsnappy-dev \
    librocksdb-dev pkg-config clang-7 lldb-7 lld-7 \
    libprotobuf-dev protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr
ENV CARGO_HOME=/usr

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=stable

WORKDIR /usr/src

###############################################################################
# Build all dependencies. Optimizing build time in next time
###############################################################################
COPY Cargo.toml ./
COPY node/Cargo.toml node/
RUN mkdir node/src/ \
    && echo "fn main() { panic!(\"if you see this, the build broke\") }" > node/src/main.rs \
    && cargo build --release \
    # Clean target from dummy project files
    && rm -rf target/release/deps/crypto-election-node*

###############################################################################
# Build project
###############################################################################
COPY node/build.rs node/build.rs
COPY node/src node/src
COPY proto proto
RUN cd node && cargo build --release && cargo install --path . \
    && rm -rf target ../target

COPY node/launch.sh ./

WORKDIR /usr/src/cfg

COPY cfg/crypto-election.yml ./

CMD ["../launch.sh"]
