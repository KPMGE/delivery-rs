# build stage
FROM rust:1.68 as build

WORKDIR /rust/src

COPY . .

RUN cargo build --release


# run stage
FROM rust:1.68

WORKDIR /simulator

COPY --from=build /rust/src/target/release/kafka-rust ./

CMD [ "./kafka-rust" ]
