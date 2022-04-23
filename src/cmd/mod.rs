pub mod cmd;
use log::info;
use rdkafka::util::get_rdkafka_version;
use structopt::StructOpt;

use self::cmd::{Action, CommandLineArgs};
use crate::application::kafka::{
  processor::{self, KafkaProcessor},
  producer::{new_kafka_producer, publish},
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
      //db::init();
      let database = connection().expect("Failed getting db connection");
      //
      publish(
        &String::from("Ola Kafka"),
        &String::from("teste"),
        &producer,
      )
      .await?;
      //
      let _kafka_processor = KafkaProcessor::new(database, producer);
      processor::consume().await?;
    }
  };
  Ok(())
}
