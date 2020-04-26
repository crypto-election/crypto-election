FROM node:13.10.1-buster

WORKDIR /usr/src/web

ENV api_host=node
ENV api_port=8000

CMD npm start -- --port=8008 --api-root=http://$api_host:$api_port