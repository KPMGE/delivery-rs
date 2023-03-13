use std::thread;
use std::time::Duration;

use rdkafka::{producer::BaseProducer, ClientConfig};
use rdkafka::producer::BaseRecord;
use crate::models::{Route, PartialRoutePosition};

pub fn send_route_to_kafka(route: &Route, topic: &str) {
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

        thread::sleep(Duration::from_millis(500));
    }
}
