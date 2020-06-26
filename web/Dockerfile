FROM node:13.14.0-buster as build

WORKDIR /usr/src/web

ADD web/package.json ./
RUN npm install

ADD proto/ ../proto/
ADD web/ ./

RUN npm run build-prod


FROM node:13.14.0-alpine

WORKDIR /usr/src

COPY --from=build /usr/src/web/dist .

ENV api_host=node
ENV api_port=8000
ENV port=8008

CMD node server.js --port=$port --api-root=http://$api_host:$api_port