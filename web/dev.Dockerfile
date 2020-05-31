FROM node:13.14.0-alpine

WORKDIR /usr/src/web

ENV api_host=node
ENV api_port=8000
ENV port=8008

CMD npm start -- --port=$port --api-root=http://$api_host:$api_port