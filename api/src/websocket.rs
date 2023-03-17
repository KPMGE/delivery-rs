use ws::{Handler, Message, Result, Sender};
use crate::models::{Route, RouteMessage};
use crate::kafka;

pub struct WebSocketServer {
    pub out: Sender,
}

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {

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
            "positions" => self.out.send("hey, here it is your position!").unwrap(),
            _ => self.out.send("hey, there is no topic for that request").unwrap()
        }

        Ok(())
    }
}
