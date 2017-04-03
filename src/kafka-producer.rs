extern crate kafka;
use kafka::producer::{Producer, Record};

fn create_producer() -> Producer {
    Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .create()
        .unwrap()
}

fn main() {
    let mut producer = create_producer();
    producer.send(&Record::from_value("test", "hello from rust experimental producer")).unwrap();
}
