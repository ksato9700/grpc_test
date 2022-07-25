FROM node:16-alpine as builder
WORKDIR /work
COPY package.json ./
RUN npm install
# COPY helloworld /usr/local/lib/python3.10/site-packages/helloworld

FROM node:16-alpine
WORKDIR /work
COPY --from=builder /work/node_modules /work/node_modules
COPY proto ./proto
COPY package.json server.js helloworld.js ./
CMD ["node", "server.js"]
