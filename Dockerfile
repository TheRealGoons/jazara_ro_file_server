FROM debian AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian

COPY --from=builder \
  /target/release/dumb_file_server \
  /usr/local/bin/

WORKDIR /root

ENV ROCKET_ADDRESS=0.0.0.0

COPY .env .env

EXPOSE 8000

CMD /usr/local/bin/dumb_file_server
