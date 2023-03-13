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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialRoutePosition {
    id: i32,
    client_id: i32,
    position: [f64; 2],
    finished: bool 
}

fn main() {
    let fictional_route = Route {
        id: 1,
        client_id: 1,
        positions: vec![
            Position {
                lat: 10.2,
                lng: 23.1
            },
            Position {
                lat: 10.2,
                lng: 23.1
            },
            Position {
                lat: 10.2,
                lng: 23.1
            },
            Position {
                lat: 10.2,
                lng: 23.1
            },
            Position {
                lat: 10.2,
                lng: 23.1
            }
        ]
    };

    // kafka config object
    // let consumer: BaseConsumer = ClientConfig::new()
    //     .set("bootstrap.servers", "host.docker.internal:9094")
    //     .set("group.id", "my-group")
    //     .create()
    //     .expect("invalid client config");

    // consumer
    //     .subscribe(&["test"])
    //     .expect("error when subscribing to the topics");

    // println!("starting listening on topics: ");
    // thread::spawn(move || loop {
    //     for msg_res in consumer.iter() {
    //         let msg = msg_res.unwrap();
    //         let value = msg.payload().unwrap();
    //         let value_str = str::from_utf8(value).unwrap();
    //         let received_route = serde_json::from_str::<Route>(value_str).expect("error when parsing json route");

    //         println!("received value: {:?}", received_route);
    //     }
    // });

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("invalid client config");

    println!("sending positions of fictional_route: ");
    for (idx, pos) in fictional_route.positions.iter().enumerate() {
        let mut partial_route = PartialRoutePosition {
            id: fictional_route.id,
            client_id: fictional_route.client_id,
            position: [pos.lat, pos.lng],
            finished: false
        };

        if idx == fictional_route.positions.len() - 1 {
            partial_route.finished = true;
        }

        let route_str = serde_json::to_string_pretty(&partial_route).expect("error when stringfying route");
        producer.send(
            BaseRecord::to("test")
                .key(&format!("key-{}", 1))
                .payload(&route_str)
        ).expect("failed to send message");

        thread::sleep(Duration::from_secs(1));
    }
}
