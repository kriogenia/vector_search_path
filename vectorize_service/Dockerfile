FROM rust:1.69 AS builder

USER root

RUN apt-get update  \
    && apt-get install -y curl build-essential ca-certificates tzdata net-tools pkg-config libssl-dev openssl python3-pip
RUN curl -XGET https://download.pytorch.org/libtorch/cu117/libtorch-cxx11-abi-shared-with-deps-1.13.0%2Bcu117.zip --output libtorch.zip
RUN cp libtorch.zip /lib && unzip /lib/libtorch
RUN export LIBTORCH=/lib/libtorch
RUN export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

RUN cargo new --bin vectorize_service
WORKDIR /vectorize_service

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN rm src/*.rs
COPY ./src ./src

RUN cargo build --release

EXPOSE 3000

CMD [ "cargo", "run", "--release" ]
