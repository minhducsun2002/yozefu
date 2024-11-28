FROM rust:alpine

RUN apk update --no-cache && apk add make
WORKDIR /tmp/build
COPY . .
RUN make module.wasm