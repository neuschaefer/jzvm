# See .forgejo/README.md
FROM debian:trixie

RUN apt-get -o APT::Sandbox::User=root update && apt-get -o APT::Sandbox::User=root install -y rustc cargo git

ADD . /jzvm
WORKDIR /jzvm

# Pick up the cargo index from the host, to save some time and get around the
# possibility of connection timeouts when cargo tries to update the index.
RUN mkdir /root/.cargo/registry
ADD ~/.cargo/registry/index /root/.cargo/registry/index

RUN cargo build --all-targets && cargo build --all-targets --release
