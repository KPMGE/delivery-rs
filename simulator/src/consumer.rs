use std::cell::RefCell;
use std::str;

use crate::models::{KafkaRouteRequest, Route};
use crate::producer::send_route_to_kafka;

const KAFKA_POSITION_TOPIC: &str = "positions";

pub fn consume_route(value_str: String) {
    let received_route = serde_json::from_str::<KafkaRouteRequest>(value_str.as_str())
        .expect("error when parsing json kafka route");

    let route = Route {
        client_id: received_route.client_id,
        route_id: received_route.route_id,
        positions: RefCell::new(Vec::new()),
    };

    route.load_positions(format!("destinations/{}.txt", route.route_id).as_str());
    println!("sending positions of fictional_route: ");
    send_route_to_kafka(&route, KAFKA_POSITION_TOPIC);
}
