FROM rust:1.77.2 as build

RUN USER=root cargo new --bin actix_web_api
WORKDIR /actix_web_api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/actix_web_api*
RUN cargo build --release

FROM rust:1.77.2

WORKDIR /actix_web_api

COPY --from=build /actix_web_api/target/release/actix_web_api .

EXPOSE 8080

CMD ["./actix_web_api"]