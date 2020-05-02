FROM rust:1.43.0-buster as build

RUN mkdir -p /quantum/src && mkdir /app

COPY ./src/* /quantum/src/
COPY ./Cargo.* /quantum/

WORKDIR /quantum
RUN cargo build --release && mv "./target/release/quantum" /app

FROM debian:bullseye-20200422

COPY --from=build /app /app
COPY ./boot.sh /
RUN chmod +x ./boot.sh
ENV PATH /app:$PATH

CMD ["/boot.sh"]