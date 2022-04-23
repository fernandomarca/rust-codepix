use log::info;
use rdkafka::{
  producer::{FutureProducer, FutureRecord},
  ClientConfig,
};
use std::{error::Error, time::Duration};

pub fn new_kafka_producer() -> FutureProducer {
  let producer = ClientConfig::new()
    .set("bootstrap.servers", "kafka:9092")
    .create()
    .expect("Producer creation error");
  producer
}

pub async fn publish(
  msg: &String,
  topic: &String,
  producer: &FutureProducer,
) -> Result<(), Box<dyn Error>> {
  //
  let futures = (0..5)
    .map(|i| async move {
      let delivery_status = producer
        .send(
          FutureRecord {
            topic: &topic,
            partition: None,
            payload: Some(&format!("Message {}", msg)),
            key: Some(&format!("Key {}", i)),
            timestamp: None,
            headers: None,
          },
          Duration::from_secs(0),
        )
        .await;

      // This will be executed when the result is received.
      info!("Delivery status for message {} received", i);
      delivery_status
    })
    .collect::<Vec<_>>();

  // This loop will wait until all delivery statuses have been received.
  for future in futures {
    info!("Future completed. Result: {:?}", future.await);
  }

  Ok(())
}
