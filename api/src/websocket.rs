use ws::{Handler, Message, Result, Sender};
use rdkafka::producer::Producer;
use crate::models::{Route, RouteMessage};
use rdkafka::{producer::{BaseProducer, BaseRecord}, ClientConfig};

pub struct WebSocketServer {
    pub out: Sender,
}

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {

        let text = msg.as_text().unwrap();

        println!("got message: {:?}", text);

        let (topic, json_route) = text.split_once(" ").unwrap();


        let kafka_bootstrap_server = std::env::var("KAFKA_BOOTSTRAP_SERVER")
            .expect("KAFKA_BOOTSTRAP_SERVER is not set!");

        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_bootstrap_server)
            .create()
            .expect("invalid client config");

        match topic {
            "new-route" => { 
                let route_ms = serde_json::from_str::<RouteMessage>(json_route).expect("error when parsing route");
                let route = Route { 
                    route_id: route_ms.route_id,
                    client_id: self.out.connection_id().to_string()
                };

                let route_str = serde_json::to_string(&route).expect("error while parsing json route");
                let topic = std::env::var("KAFKA_ROUTE_TOPIC").expect("KAFKA_ROUTE_TOPIC must be set");

                println!("TOPIC: {}", topic);

                self.out.send(format!("bro, i'll send the following message to kafka: {:?}", route_str)).unwrap();

                producer.send(
                    BaseRecord::to(topic.as_str())
                        .key(&route.client_id.to_string())
                        .payload(&route_str)
                ).expect("failed to post on kafka topic");


                producer.flush(std::time::Duration::from_secs(10));

                println!("message sent to kafka!");
            },
            "positions" => self.out.send("hey, here it is your position!").unwrap(),
            _ => self.out.send("hey, there is no topic for that request").unwrap()
        }

        Ok(())
    }
}
