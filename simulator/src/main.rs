use rdkafka::config::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::Message;
use rdkafka::producer::BaseProducer;
use rdkafka::producer::BaseRecord;
use std::str;
use std::thread;
use std::time::Duration;

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

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "host.docker.internal:9094")
        .create()
        .expect("invalid client config");

    for i in 0..100 {
        println!("sending message: ");

        producer.send(
            BaseRecord::to("test")
                .key(&format!("key-{}", i))
                .payload(&format!("value-{}", i))
        ).expect("failed to send message");

        thread::sleep(Duration::from_secs(3));
    }
}
