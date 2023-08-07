use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use serde_json::json;
use serde_json::Value;

fn create_json() -> Value {
    let data = json!(
        {
            "metadata": {
                "id": "saa9sd78987sd-asd",
                "timestamp": "2023-08-15T07:15:07.844Z",
            },
            "Company": "AXZ",
            "Code": "3280",
            "Date": "2023-08-15",
            
        });
    data
}


fn main() {
    let data = create_json().clone();
    let valor = serde_json::to_string(&data).unwrap();
    println!("  valor json = {}", valor);

    let mut producer =
        Producer::from_hosts(vec!("localhost:9094".to_owned()))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

    for _ in 0..2 {
        //without a key
        //producer.send(&Record::from_value("topic1", valor.to_string())).unwrap();
        let key = "10";
        producer.send(&Record::from_key_value("topic1", key, valor.as_bytes())).unwrap();
    }

}
