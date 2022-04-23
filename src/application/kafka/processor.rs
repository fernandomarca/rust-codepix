use crate::infrastructure::db::DbConnection;
use log::{info, warn};
use rdkafka::{
  config::RDKafkaLogLevel,
  consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
  error::KafkaResult,
  producer::FutureProducer,
  ClientConfig, ClientContext, Message, TopicPartitionList,
};

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
  fn pre_rebalance(&self, rebalance: &Rebalance) {
    info!("Pre rebalance {:?}", rebalance);
  }

  fn post_rebalance(&self, rebalance: &Rebalance) {
    info!("Post rebalance {:?}", rebalance);
  }

  fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
    info!("Committing offsets: {:?}", result);
  }
}

// A type alias with your custom consumer can be created for convenience.
type TesteConsumer = StreamConsumer<CustomContext>;

pub struct KafkaProcessor {
  database: DbConnection,
  producer: FutureProducer,
}

impl KafkaProcessor {
  pub fn new(database: DbConnection, producer: FutureProducer) -> KafkaProcessor {
    KafkaProcessor { database, producer }
  }
}

pub async fn consume() -> Result<(), Box<dyn std::error::Error>> {
  let context = CustomContext;
  let consumer: TesteConsumer = ClientConfig::new()
    .set("group.id", "consumergroup")
    .set("bootstrap.servers", "kafka:9092")
    //.set("enable.partition.eof", "false")
    //.set("session.timeout.ms", "6000")
    //.set("enable.auto.commit", "true")
    //.set("statistics.interval.ms", "30000")
    .set("auto.offset.reset", "latest")
    .set_log_level(RDKafkaLogLevel::Debug)
    .create_with_context(context)
    .expect("Consumer creation failed");
  //
  let topics = vec!["teste"];
  consumer
    .subscribe(&topics)
    .expect("Can't subscribe to specified topics");
  //
  loop {
    match consumer.recv().await {
      Err(e) => warn!("Kafka error: {}", e),
      Ok(m) => {
        let payload = match m.payload_view::<str>() {
          None => "",
          Some(Ok(s)) => s,
          Some(Err(e)) => {
            warn!("Error while deserializing message payload: {:?}", e);
            ""
          }
        };
        //log
        info!(
          "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
          m.key(),
          payload,
          m.topic(),
          m.partition(),
          m.offset(),
          m.timestamp()
        );
        //
        // if let Some(headers) = m.headers() {
        //   for header in headers.iter() {
        //     info!("  Header {:#?}: {:?}", header.key, header.value);
        //   }
        // }
        //
        consumer.commit_message(&m, CommitMode::Async).unwrap();
      }
    };
  }
}
