use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    Success,
    UserCodeError,
    ScalingError,
    SparkError,
    SparkOomError,
    NetworkError,
    PermissionError,
    UnknownError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorClassification {
    pub error_type: ErrorType,
    pub messages: Vec<String>,
}
