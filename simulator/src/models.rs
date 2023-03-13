use std::{fs::File, cell::RefCell, io::BufReader};
use serde::{Serialize, Deserialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaRouteRequest {
    pub route_id: String,
    pub client_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub lat: f64,
    pub lng: f64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub route_id: String,
    pub client_id: String,
    pub positions: RefCell<Vec<Position>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialRoutePosition {
    pub route_id: String,
    pub client_id: String,
    pub position: [f64; 2],
    pub finished: bool 
}

impl Route {
    pub fn load_positions(&self, file_path: &str) {
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
