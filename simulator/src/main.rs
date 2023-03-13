use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::producer::BaseProducer;
use rdkafka::producer::BaseRecord;
use std::str;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    lat: f64,
    lng: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Route {
    id: i32,
    client_id: i32,
    positions: Vec<Position>
}

fn main() {
    // kafka config object
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .set("group.id", "my-group")
        .create()
        .expect("invalid client config");

    consumer
        .subscribe(&["test"])
        .expect("error when subscribing to the topics");

    println!("starting listening on topics: ");
    thread::spawn(move || loop {
        for msg_res in consumer.iter() {
            let msg = msg_res.unwrap();
            let value = msg.payload().unwrap();
            let value_str = str::from_utf8(value).unwrap();
            let received_route = serde_json::from_str::<Route>(value_str).expect("error when parsing json route");

            println!("received value: {:?}", received_route);
        }
    });

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("invalid client config");

    for i in 0..100 {
        println!("sending message: {}", i);

        let route = Route {
            id: i,
            client_id: i,
            positions: vec![
                Position {
                    lat: 10.34,
                    lng: 12.23
                }
            ]
        };

        let route_str = serde_json::to_string_pretty(&route).expect("error when stringfying route");
        producer.send(
            BaseRecord::to("test")
                .key(&format!("key-{}", i))
                .payload(&route_str)
        ).expect("failed to send message");

        thread::sleep(Duration::from_secs(3));
    }
}
