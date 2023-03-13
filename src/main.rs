use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use std::str;
use std::thread;

fn main() {
    // kafka config object
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .set("group.id", "my-group")
        .create()
        .expect("invalid client config");

    consumer
        .subscribe(&["test"])
        .expect("error when subscribing to the topics");

    println!("starting listening on topics: ");
    thread::spawn(move || loop {
        for msg_res in consumer.iter() {
            let msg = msg_res.unwrap();
            let value = msg.payload().unwrap();
            let value_str = str::from_utf8(value).unwrap();
            println!("received value: {:?}", value_str);
        }
    });

    loop {}
}
