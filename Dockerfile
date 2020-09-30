FROM rustlang/rust:nightly
MAINTAINER Michael Balitsky

WORKDIR /usr/src/proxy_rust
COPY . .
RUN cargo install --path .
EXPOSE 1337
CMD ["proxy_rust", "x"]