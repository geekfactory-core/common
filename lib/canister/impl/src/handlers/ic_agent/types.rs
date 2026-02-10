use ic_cdk::call::RejectCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectResponse {
    /// The [reject code](https://smartcontracts.org/docs/interface-spec/index.html#reject-codes) returned by the replica.
    pub reject_code: RejectCode,
    /// The rejection message.
    pub reject_message: String,
    /// The optional [error code](https://smartcontracts.org/docs/interface-spec/index.html#error-codes) returned by the replica.
    #[serde(default)]
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status")]
pub enum QueryResponse {
    #[serde(rename = "replied")]
    Replied { reply: CallReply },
    #[serde(rename = "rejected")]
    Rejected(RejectResponse),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallReply {
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ReadStateResponse {
    #[serde(with = "serde_bytes")]
    pub certificate: Vec<u8>,
}
