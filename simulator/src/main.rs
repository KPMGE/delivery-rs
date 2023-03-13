use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::producer::BaseProducer;
use rdkafka::producer::BaseRecord;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KafkaRouteRequest {
    route_id: String,
    client_id: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    lat: f64,
    lng: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Route {
    route_id: String,
    client_id: String,
    positions: RefCell<Vec<Position>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartialRoutePosition {
    route_id: String,
    client_id: String,
    position: [f64; 2],
    finished: bool 
}

impl Route {
    fn load_positions(&self, file_path: &str) {
        let file = File::open(file_path).expect("error when opening route file");
        let reader = BufReader::new(file);

        for line_res in reader.lines() {
            let line = line_res.unwrap();
            let (lat, lng) = line.split_once(",").unwrap();

            self.positions.borrow_mut().push(Position {
                lat: str::parse::<f64>(lat).unwrap(),
                lng: str::parse::<f64>(lng).unwrap()
            });
        }
    }
}

fn main() {
    // create fictional_route with some id and client_id
    let fictional_route = Route {
        route_id: "1".to_string(),
        client_id: "1".to_string(),
        positions: RefCell::new(vec![])
    };

    // load positions from file
    fictional_route.load_positions("destinations/1.txt");

    // kafka config object
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .set("group.id", "my-group")
        .create()
        .expect("invalid client config");

    const KAFKA_ROUTE_TOPIC: &str = "route";
    consumer
        .subscribe(&[KAFKA_ROUTE_TOPIC])
        .expect("error when subscribing to the topics");

    println!("starting listening on topics: ");
    thread::spawn(move || loop {
        for msg_res in consumer.iter() {
            let msg = msg_res.unwrap();
            let value = msg.payload().unwrap();
            let value_str = str::from_utf8(value).unwrap();
            let received_route = serde_json::from_str::<KafkaRouteRequest>(value_str)
                .expect("error when parsing json kafka route");

            let route = Route {
                client_id: received_route.client_id,
                route_id: received_route.route_id,
                positions: RefCell::new(Vec::new())
            };

            route.load_positions(format!("destinations/{}.txt", route.route_id).as_str());

            println!("sending positions of fictional_route: ");

            const KAFKA_POSITION_TOPIC: &str = "positions";
            send_route_to_kafka(&route, KAFKA_POSITION_TOPIC);
        }
    });

    loop {}
}

fn send_route_to_kafka(route: &Route, topic: &str) {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("invalid client config");

    for (idx, pos) in route.positions.borrow().iter().enumerate() {
        let mut partial_route = PartialRoutePosition {
            route_id: route.route_id.clone(),
            client_id: route.client_id.clone(),
            position: [pos.lat, pos.lng],
            finished: false
        };

        if idx == route.positions.borrow().len() - 1 {
            partial_route.finished = true;
        }

        let route_str = serde_json::to_string_pretty(&partial_route).expect("error when stringfying route");
        producer.send(
            BaseRecord::to(topic)
                .key(&route.route_id)
                .payload(&route_str)
        ).expect("failed to send message");

        thread::sleep(Duration::from_millis(200));
    }
}
