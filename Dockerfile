FROM rust:1.59.0
WORKDIR /rust/src
ENV PATH="/rust/bin:${PATH}"
RUN apt-get update
CMD ["tail", "-f", "/dev/null"]