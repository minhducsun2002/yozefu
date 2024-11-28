FROM tinygo/tinygo:latest

USER root
RUN apt update && apt install make
USER tinygo
WORKDIR /tmp/build
COPY --chown=tinygo . .
RUN make module.wasm