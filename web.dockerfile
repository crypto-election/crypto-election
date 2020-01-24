FROM ubuntu:18.04

 RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get update \
    && apt-get install -y curl git gcc cpp \
    libssl-dev pkg-config libsodium-dev libsnappy-dev librocksdb-dev \
    libprotobuf-dev protobuf-compiler && curl -sL https://deb.nodesource.com/setup_8.x | bash \
    && apt-get install -y nodejs

WORKDIR /usr/src

ADD core/src/proto/ proto/
ADD web/ web/

WORKDIR /usr/src/web

RUN npm install && npm run build

CMD ["npm", "start", "--", "--port=8008", "--api-root=http://127.0.0.1:8000"]