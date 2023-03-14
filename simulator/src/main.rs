use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use std::str;
use std::thread;
use dotenv::dotenv;

mod producer;
mod consumer;
mod models;

fn main() {
    // load environment variables
    dotenv().ok();

    let kafka_consumer_grup_id = std::env::var("KAFKA_CONSUMER_GROUP_ID")
        .expect("KAFKA_CONSUMER_GROUP_ID is not set!");
    let kafka_bootstrap_server = std::env::var("KAFKA_BOOTSTRAP_SERVER")
        .expect("KAFKA_BOOTSTRAP_SERVER is not set!");
    let kafka_route_topic = std::env::var("KAFKA_ROUTE_TOPIC")
        .expect("KAFKA_ROUTE_TOPIC is not set!");

    // kafka config object
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_bootstrap_server)
        .set("group.id", kafka_consumer_grup_id)
        .create()
        .expect("invalid client config");

    consumer
        .subscribe(&[kafka_route_topic.as_str()])
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
