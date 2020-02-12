FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get update \
    && apt-get install -y curl git gcc cpp \
    libssl-dev pkg-config libsodium-dev libsnappy-dev \
    libprotobuf-dev protobuf-compiler \
    && curl -sL https://deb.nodesource.com/setup_10.x | bash \
    && apt-get install -y nodejs

WORKDIR /usr/src

COPY proto/ proto/
COPY web/ web/

WORKDIR /usr/src/web

RUN npm install && npm run build

ENV API_ADDR=127.0.0.1
ENV API_PORT=8000

ENV PORT=8008

CMD ["npm", "start", "--", "--port=$PORT", "--api-root=http://$API_ADDR:$API_PORT"]