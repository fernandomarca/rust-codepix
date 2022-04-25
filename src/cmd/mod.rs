pub mod cmd;
use log::info;
use rdkafka::util::get_rdkafka_version;
use std::env;
use structopt::StructOpt;

use self::cmd::{Action, CommandLineArgs};
use crate::application::kafka::{
  processor::{self, KafkaProcessor},
  producer::new_kafka_producer,
};
use crate::{
  application::grpc::pixgrpc::{server_grpc, MyPix},
  infrastructure::db::{self, connection},
};

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
  let CommandLineArgs { action, port } = CommandLineArgs::from_args();

  match action {
    Action::Start => {
      db::init();
      let pix_service = MyPix {};
      server_grpc(pix_service, port).await?;
    }
    Action::Kafka => {
      let (version_n, version_s) = get_rdkafka_version();
      info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
      //
      let producer = new_kafka_producer();
      //
      let database = connection().expect("Failed getting db connection");
      //
      let _kafka_processor = KafkaProcessor::new(database, producer);
      processor::consume().await?;
    }
    Action::All => {
      //start gRpc
      async fn start_grpc(port: String) -> Result<(), Box<dyn std::error::Error>> {
        db::init();
        let pix_service = MyPix {};
        let result = server_grpc(pix_service, port).await;
        result
      }
      // start kafka
      async fn start_kafka() -> Result<(), Box<dyn std::error::Error>> {
        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
        let producer = new_kafka_producer();
        let database = connection().expect("Failed getting db connection");
        //topics
        let kafkaTransactionTopic =
          env::var("kafkaTransactionTopic").expect("env kafkaTransactionTopic eror");
        let kafkaTransactionConfirmationTopic = env::var("kafkaTransactionConfirmationTopic")
          .expect("env kafkaTransactionConfirmationTopic eror");
        let topics: Vec<&str> = vec![
          kafkaTransactionTopic.as_ref(),
          kafkaTransactionConfirmationTopic.as_ref(),
        ];
        //envs
        let group_id = env::var("kafkaConsumerGroupId").expect("env kafkaConsumer eror");
        let bootstrap = env::var("kafkaBootstrapServers").expect("env kafkaBootstrapServers eror");
        //
        let kafka_processor: KafkaProcessor = KafkaProcessor::new(database, producer);
        KafkaProcessor::consume(&topics, group_id, bootstrap).await?;
        Ok(())
      }

      let grpc = tokio::spawn(async move {
        let _result = start_grpc(port).await;
      });
      let kafka = tokio::spawn(async move {
        let _result = start_kafka().await;
      });

      let _result = tokio::join!(grpc, kafka);
    }
  };
  Ok(())
}
