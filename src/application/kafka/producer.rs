use log::info;
use rdkafka::{
  producer::{FutureProducer, FutureRecord},
  ClientConfig,
};
use std::{error::Error, time::Duration};

pub fn new_kafka_producer() -> FutureProducer {
  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "kafka-0:9092")
    .create()
    .expect("Producer creation error");
  producer
}

pub async fn publish(
  msg: String,
  topic: String,
  producer: FutureProducer,
) -> Result<(), Box<dyn Error>> {
  let message = FutureRecord::to(&topic)
    .payload(msg.as_bytes())
    .key("codepix-1");

  let delivery_status = producer.send(message, Duration::from_secs(0)).await;

  let status = match delivery_status {
    Ok(status) => status,
    Err(e) => todo!(),
  };

  info!("Delivery status for message received: {:?}", status);
  Ok(())
}

// let (version_n, version_s) = get_rdkafka_version();
// info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
