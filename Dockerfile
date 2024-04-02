# https://github.com/docker-library/repo-info/blob/master/repos/alpine/tag-details.md#alpine3190
# https://hub.docker.com/layers/library/alpine/3.19.1/images/sha256-6457d53fb065d6f250e1504b9bc42d5b6c65941d57532c072d929dd0628977d0?context=explore
FROM alpine:3.19.1@sha256:c5b1261d6d3e43071626931fc004f70149baeba2c8ec672bd4f27761f8e1ad6b AS base
ENV TERM xterm-256color
RUN apk add --no-cache rust cargo pkgconfig openssl-dev

#
# pre-build, download cargo.io repository
# used to speed-up build process
#
FROM base AS pre-build
WORKDIR /builder

RUN mkdir -p .cargo && \
    mkdir -p src && \
    mkdir -p examples

RUN printf 'fn main() { println!("Hello, world!"); }' > examples/dummy.rs

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo install --path . --root ./tmp --example dummy
RUN rm examples/dummy.rs

#
# builder state
#
FROM pre-build AS builder
WORKDIR /builder

COPY .cargo .cargo
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo --version
RUN cargo install --path . --root .

#
# test stage
#
FROM base AS test
RUN apk add --no-cache openrc

CMD openrc default && \
    cd /builder && \
    cargo test

#
# create clean image with chat application only
#
FROM alpine:3.19.1@sha256:c5b1261d6d3e43071626931fc004f70149baeba2c8ec672bd4f27761f8e1ad6b AS runtime
# libgcc is required to run rust applications
RUN apk add --no-cache libgcc

COPY --from=builder /builder/bin/chat /usr/local/bin/chat

RUN adduser user -D -G users
USER user

EXPOSE 8080

WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/chat"]
