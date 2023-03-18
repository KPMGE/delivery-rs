use std::thread;

use std::str;
use rdkafka::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::Message;

use ws::{Handler, Result, Sender};
use crate::models::KafkaPosition;
use crate::models::{Route, RouteMessage};
use crate::kafka;

pub struct WebSocketServer {
    pub out: Sender,
}

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: ws::Message) -> Result<()> {

        let text = msg.as_text().unwrap();

        println!("got message: {:?}", text);

        let (topic, json_route) = text.split_once(" ").unwrap();


        match topic {
            "new-route" => { 
                let route_ms = serde_json::from_str::<RouteMessage>(json_route)
                    .expect("error when parsing route");
                let route = Route { 
                    route_id: route_ms.route_id,
                    client_id: self.out.connection_id().to_string()
                };

                let route_str = serde_json::to_string(&route)
                    .expect("error while parsing json route");
                let topic = std::env::var("KAFKA_ROUTE_TOPIC")
                    .expect("KAFKA_ROUTE_TOPIC must be set");

                self.out.send(format!("bro, i'll send the following message to kafka: {:?}", route_str)).unwrap();

                kafka::post_route_into_kafka(
                    route, 
                    topic.as_str(), 
                    route_str.as_str()
                );
            },
            "positions" => {
                let kafka_consumer_grup_id = std::env::var("KAFKA_CONSUMER_GROUP_ID")
                    .expect("KAFKA_CONSUMER_GROUP_ID is not set!");
                let kafka_bootstrap_server = std::env::var("KAFKA_BOOTSTRAP_SERVER")
                    .expect("KAFKA_BOOTSTRAP_SERVER is not set!");
                let kafka_position_topic = std::env::var("KAFKA_POSITION_TOPIC")
                    .expect("KAFKA_POSITION_TOPIC is not set!");

                self.out.send("hey, i'll start listening on kafka, ok").unwrap();

                let sender = self.out.clone();

                thread::spawn(move || {
                    let consumer: BaseConsumer = ClientConfig::new()
                        .set("bootstrap.servers", kafka_bootstrap_server)
                        .set("group.id", kafka_consumer_grup_id)
                        .create()
                        .expect("invalid client config");

                    consumer
                        .subscribe(&[kafka_position_topic.as_str()])
                        .expect("error when subscribing to the topics");

                    println!("listening on positions topic...");
                    for msg_res in consumer.iter() {
                        let msg = msg_res.unwrap();
                        let value = msg.payload().unwrap().clone();
                        let value_string = str::from_utf8(value).unwrap().to_string();

                        // println!("val str: {}", value_string);

                        let position = serde_json::from_str::<KafkaPosition>(value_string.as_str()).unwrap();
                        println!("received position: {:?}", position);

                        sender.send(format!("{}", value_string)).unwrap();
                    }
                });
            },
            _ => self.out.send("hey, there is no topic for that request").unwrap()
        }

        Ok(())
    }
}
