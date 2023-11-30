# See .forgejo/README.md
FROM debian:trixie

RUN apt-get -o APT::Sandbox::User=root update && apt-get -o APT::Sandbox::User=root install -y rustc cargo git

ADD . /jzvm
WORKDIR /jzvm

RUN cargo build --all-targets && cargo build --all-targets --release
