use std::thread;
use serde::{Serialize, Deserialize};
use ws::{listen, Handler, Message, Result, Sender};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RouteMessage {
    route_id: i32,
}

#[derive(Debug)]
struct Route {
    route_id: i32,
    client_id: u32
}

struct WebSocketServer {
    out: Sender,
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


#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod repository;

use repository::Repository;
use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    // load environment variables
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let repo = Repository::new(db_url.as_str())
        .await
        .expect("error while connecting to the database");

    thread::spawn(|| {
        let url = "127.0.0.1:3012";
        println!("websocket server on: ws://{}", url);
        listen(url, |out| {
            WebSocketServer { out }
        })
        .unwrap();
    });

    rocket::build()
        .manage::<Repository>(repo).mount("/", routes![
            routes::get_all_routes,
            routes::add_route, 
       ],
    )
}
