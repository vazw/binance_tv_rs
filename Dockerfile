FROM rust:latest
WORKDIR /usr/src/binance_tv_rs
COPY . .
RUN cargo build --release

ENV TZ=Asia/Bangkok
ENV BINANCE_API="binance-api"
ENV BINANCE_SEC="binance-sec-api"
ENV PASSPHARSE="225420"
ENV LINE_TOKEN="line token"
ENV FREE_BALANCE=50
CMD ["./target/release/binance_tv_rs"]
