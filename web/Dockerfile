FROM node:13.10.1-buster

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
            libprotobuf-dev \
            libsodium-dev \
            pkg-config \
            protobuf-compiler \
            software-properties-common;

WORKDIR /usr/src
ADD proto/ proto/
ADD web/package.json web/

WORKDIR /usr/src/web
RUN npm install

ADD web .
RUN npm run build

ENV api_host=node
ENV api_port=8000

CMD npm start -- --port=8008 --api-root=http://$api_host:$api_port