FROM rust:1.42.0-buster

ENV ROCKSDB_LIB_DIR=/usr/lib
ENV SNAPPY_LIB_DIR=/usr/lib/x86_64-linux-gnu

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
            build-essential \
            clang-7 \
            gdb \
            gdbserver \
            libprotobuf-dev \
            librocksdb-dev \
            libsnappy-dev \
            libsodium-dev \
#            lld-7 \
#            lldb-7 \
            pkg-config \
            protobuf-compiler \
            software-properties-common; \
    rm -rf /var/lib/apt/lists/*;

###############################################################################
# Build all dependencies. Optimizing build time in next time
###############################################################################
WORKDIR /usr/src
COPY Cargo.toml ./
COPY node/Cargo.toml node/
RUN mkdir node/src/ \
    && echo "fn main() { panic!(\"if you see this, the build broke\") }" > node/src/main.rs \
    && cargo build \
    # Clean target from dummy project files
    && rm -rf target/debug/deps/crypto-election-node*

###############################################################################
# Build project
###############################################################################
COPY node/build.rs node/build.rs
COPY node/src node/src
COPY proto proto
RUN cd node && cargo build

COPY node/launch*.sh ./

WORKDIR /usr/src/cfg

COPY cfg/crypto-election.yml ./

CMD ["../launch-dev.sh"]
