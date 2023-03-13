use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use std::str;
use std::thread;

mod producer;
mod consumer;
mod models;

const KAFKA_ROUTE_TOPIC: &str = "route";

fn main() {
    // kafka config object
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .set("group.id", "my-group")
        .create()
        .expect("invalid client config");

    consumer
        .subscribe(&[KAFKA_ROUTE_TOPIC])
        .expect("error when subscribing to the topics");

    println!("starting listening on topics: ");

    for msg_res in consumer.iter() {
        let msg = msg_res.unwrap();
        let value = msg.payload().unwrap().clone();
        let value_string = str::from_utf8(value).unwrap().to_string();

        thread::spawn(move || {
            consumer::consume_route(value_string);
        });
    }
}
