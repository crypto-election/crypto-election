FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:maarten-fonville/protobuf \
    && apt-get update \
    && apt-get install -y curl gcc cpp \
    libssl-dev pkg-config libsodium-dev libsnappy-dev \
    libprotobuf-dev protobuf-compiler \
    && curl -sL https://deb.nodesource.com/setup_10.x | bash \
    && apt-get install -y nodejs

WORKDIR /usr/src

ADD proto/ proto/
ADD web/ web/

WORKDIR /usr/src/web

RUN npm install && npm run build

ENV api_host=node
ENV api_port=8000

CMD npm start -- --port=8008 --api-root=http://$api_host:$api_port