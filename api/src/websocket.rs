use ws::{Handler, Message, Result, Sender};
use crate::models::{Route, RouteMessage};

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
                let route_ms = serde_json::from_str::<RouteMessage>(json_route).expect("error when parsing route");
                let route = Route { 
                    route_id: route_ms.route_id,
                    client_id: self.out.connection_id()
                };
                self.out.send(format!("bro, i'll send the following message to kafka: {:?}", route)).unwrap();
            },
            "positions" => self.out.send("hey, here it is your position!").unwrap(),
            _ => self.out.send("hey, there is no topic for that request").unwrap()
        }

        Ok(())
    }
}
