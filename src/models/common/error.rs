use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum HetznerErrorCode {
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "invalid_input")]
    InvalidInput,
    #[serde(rename = "json_error")]
    JsonError,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "resource_limit_exceeded")]
    ResourceLimitExceeded,
    #[serde(rename = "resource_unavailable")]
    ResourceUnavailable,
    #[serde(rename = "service_error")]
    ServiceError,
    #[serde(rename = "uniqueness_error")]
    UniquenessError,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "unsupported_error")]
    UnsupportedError,
    #[serde(rename = "token_readonly")]
    TokenReadonly,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "action_failed")]
    ActionFailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerError{
    pub code: HetznerErrorCode,
    pub message: String,
}