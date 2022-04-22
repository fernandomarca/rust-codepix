pub mod cmd;

use structopt::StructOpt;

use self::cmd::{Action, CommandLineArgs};
use crate::{
  application::grpc::pixgrpc::{server_grpc, MyPix},
  infrastructure::db,
};

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
  let CommandLineArgs { action, port } = CommandLineArgs::from_args();

  match action {
    Action::Start => {
      db::init();
      let pix_service = MyPix {};
      server_grpc(pix_service, port).await?;
    }
    Action::Kafka => todo!(),
  };
  Ok(())
}
