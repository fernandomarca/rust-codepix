FROM rust:1.59.0
WORKDIR /rust
ENV PATH="/rust/bin:${PATH}"
RUN apt-get update && \
  apt-get install build-essential protobuf-compiler librdkafka-dev libssl-dev libsasl2-dev libzstd-dev -y
CMD ["tail", "-f", "/dev/null"]