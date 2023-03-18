use crate::models::{Route, KafkaPosition};
use std::str;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::Message;
use rdkafka::producer::Producer;
use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

pub fn post_route_into_kafka(route: Route, topic: &str, payload: &str) {
    let kafka_bootstrap_server =
        std::env::var("KAFKA_BOOTSTRAP_SERVER").expect("KAFKA_BOOTSTRAP_SERVER is not set!");

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_bootstrap_server)
        .create()
        .expect("invalid client config");

    producer
        .send(
            BaseRecord::to(topic)
                .key(&route.client_id.to_string())
                .payload(payload),
        )
        .expect("failed to post on kafka topic");

    producer.flush(std::time::Duration::from_secs(10));

    println!("message sent to kafka!");
}

pub fn consume_position_from_kafka() {
    todo!();
}
