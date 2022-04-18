use chrono::Timelike;

use crate::api_grpc;
use api_grpc::pixkey::PixKeyResponse;

use super::PixKeyModel;

//pub(crate)

//conversion Model for PixKeyResponse

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
