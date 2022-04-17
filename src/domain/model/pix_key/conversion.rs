use chrono::{DateTime, Timelike, Utc};

use crate::api_grpc;
use crate::{domain::model::pix_key::PixKeyModel, infrastructure::prisma_db::db::PixKeyPData};
use api_grpc::pixkey::PixKeyResponse;
//pub(crate)
pub struct PixKeyWrapper {
  pub(crate) pix_key: PixKeyModel,
}

pub struct PixKeyWrapperPrisma {
  pub pix_key: PixKeyPData,
}

//conversion PrismaData response for PixKeyModel
impl From<&PixKeyPData> for PixKeyModel {
  fn from(entity: &PixKeyPData) -> PixKeyModel {
    let pk = entity.clone();

    PixKeyModel {
      id: pk.clone().id,
      kind: pk.clone().kind,
      key: pk.clone().key,
      created_at: pk.clone().created_at.parse::<DateTime<Utc>>().unwrap(),
      updated_at: pk
        .clone()
        .updated_at
        .unwrap_or(pk.created_at.parse::<DateTime<Utc>>().unwrap().to_string())
        .parse::<DateTime<Utc>>()
        .unwrap(),
      account_id: pk.clone().account_id,
      status: pk.clone().status,
    }
  }
}

//conversion PrismaData response for PixKeyModel
impl From<PixKeyPData> for PixKeyModel {
  fn from(entity: PixKeyPData) -> PixKeyModel {
    let pk = entity.clone();
    let id = pk.clone().id;
    let kind = pk.clone().kind;
    let key = pk.clone().key;
    let created_at = pk.clone().created_at.parse::<DateTime<Utc>>().unwrap();
    let updated_at = pk
      .clone()
      .updated_at
      .unwrap_or(pk.created_at.parse::<DateTime<Utc>>().unwrap().to_string())
      .parse::<DateTime<Utc>>()
      .unwrap();
    //let account_p = pk.clone().account();
    // let account = pk.clone().account().unwrap();
    let account_id = pk.clone().account_id;
    let status = pk.clone().status;
    PixKeyModel {
      id,
      kind,
      key,
      created_at,
      updated_at,
      account_id,
      status,
    }
  }
}

impl From<PixKeyModel> for PixKeyResponse {
  fn from(px: PixKeyModel) -> PixKeyResponse {
    let created_at = prost_types::Timestamp {
      seconds: Timelike::second(&px.created_at) as i64,
      nanos: 0,
    };
    let updated_at = prost_types::Timestamp {
      seconds: Timelike::second(&px.updated_at) as i64,
      nanos: 0,
    };
    PixKeyResponse {
      id: px.id,
      kind: px.kind,
      key: px.key,
      created_at: Some(created_at),
      updated_at: Some(updated_at),
      account_id: px.account_id,
      status: px.status,
    }
  }
}

// pub struct PixKeyCreatedResult {
//   pub id: String,
//   pub status: String,
// }

// impl From<PixKeyPData> for PixKeyCreatedResult {
//   fn from(t: PixKeyPData) -> Self {
//     Self {
//       id: t.id,
//       status: t.status,
//     }
//   }
// }
