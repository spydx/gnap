use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceRequest {
    pub instance_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceResponse {
    pub status: bool,
}

impl InstanceResponse {
    pub fn create(value: bool) -> Self {
        Self { status: value }
    }
}

impl InstanceRequest {
    pub fn create(value: String) -> Self {
        Self { instance_id: value }
    }
}
