use crate::{
  api_error::ApiError,
  application::{
    factory::{pixkey_usecase_factory, transaction_usecase_factory},
    model::{parse_json, to_json},
    usecase::transaction::TransactionUseCase,
  },
  domain::model::transaction::TransactionDto,
  infrastructure::db::DbConnection,
};
use log::{info, warn};
use rdkafka::{
  config::RDKafkaLogLevel,
  consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
  error::KafkaResult,
  message::BorrowedMessage,
  producer::FutureProducer,
  ClientConfig, ClientContext, Message, TopicPartitionList,
};

use super::producer::publish;

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
  async fn process_message(&self, msg: BorrowedMessage<'_>) -> Result<(), ApiError> {
    let transactions_topic = "transactions";
    let transaction_confirmation_topic = "transaction_confirmation";

    let result = match Message::topic(&msg) {
      _transactions_topic => self.process_transaction(&msg).await,
      _transaction_confirmation_topic => self.process_transaction_confirmation(&msg),
      _ => {
        warn!("not a valid topic: {:?}", msg.payload_view::<str>());
        Err(ApiError::new(404, String::from("not a valid topic")))
      }
    };
    result
  }

  async fn process_transaction(&self, msg: &BorrowedMessage<'_>) -> Result<(), ApiError> {
    let transaction_dto = parse_json(msg);
    let transaction_usecase = transaction_usecase_factory();
    let created_transaction = transaction_usecase.register(
      transaction_dto.account_from_id,
      transaction_dto.amount,
      transaction_dto.pix_key_id_to,
      transaction_dto.description,
      None,
    )?;
    // verify diesel for return relations in one query
    let pix_key_usecase = pixkey_usecase_factory();
    let pix_key = pix_key_usecase.find_pix_by_id(&created_transaction.pix_key_id_to)?;
    let account_id_in_pixkey = pix_key.account_id;
    let account_entity = pix_key_usecase.find_account(account_id_in_pixkey)?;
    let bank_id = account_entity.bank_id;
    let bank_entity = pix_key_usecase.find_bank(bank_id)?;
    //
    // let topic = "bank" + created_transaction.PixKeyTo.Account.Bank.Code
    let topic = format!("bank{}", bank_entity.code);
    // let transaction_id = created_transaction.id;
    // let transaction_status = created_transaction.status;
    //
    let transaction_json = to_json(&created_transaction);
    publish(&transaction_json, &topic, &self.producer)
      .await
      .expect("error in publish process_transaction");
    Ok(())
  }

  fn process_transaction_confirmation(&self, msg: &BorrowedMessage) -> Result<(), ApiError> {
    let transaction = parse_json(&msg);
    let transaction_usecase = transaction_usecase_factory();
    //
    if transaction.status == "confirmed".to_string() {
      self.confirm_transaction(transaction, transaction_usecase);
    } else if transaction.status == "completed".to_string() {
      let _result = transaction_usecase.complete(transaction.id)?;
    }
    Ok(())
  }

  async fn confirm_transaction(
    &self,
    transaction: TransactionDto,
    transaction_usecase: TransactionUseCase,
  ) -> Result<(), ApiError> {
    let confirmed_transaction = transaction_usecase.confirm(transaction.id)?;
    // verify diesel for return relations in one query
    let pix_key_usecase = pixkey_usecase_factory();
    let pix_key = pix_key_usecase.find_pix_by_id(&confirmed_transaction.pix_key_id_to)?;
    let account_id_in_pixkey = pix_key.account_id;
    let account_entity = pix_key_usecase.find_account(account_id_in_pixkey)?;
    let bank_id = account_entity.bank_id;
    let bank_entity = pix_key_usecase.find_bank(bank_id)?;
    //
    // topic := "bank" + confirmedTransaction.AccountFrom.Bank.Code
    let topic = format!("bank{}", bank_entity.code);
    let transaction_json = to_json(&confirmed_transaction);
    publish(&transaction_json, &topic, &self.producer)
      .await
      .expect("error in publish transaction_confirm");
    Ok(())
  }

  pub async fn consume(
    topics: &Vec<&str>,
    group_id: String,
    bootstrap: String,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let context = CustomContext;
    //
    let consumer: TesteConsumer = ClientConfig::new()
      .set("group.id", group_id)
      .set("bootstrap.servers", bootstrap)
      //.set("enable.partition.eof", "false")
      //.set("session.timeout.ms", "6000")
      //.set("enable.auto.commit", "true")
      //.set("statistics.interval.ms", "30000")
      .set("auto.offset.reset", "latest")
      .set_log_level(RDKafkaLogLevel::Debug)
      .create_with_context(context)
      .expect("Consumer creation failed");
    //
    consumer
      .subscribe(topics)
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
}
