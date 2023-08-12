use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    pub connections: Option<Vec<Connection>>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub bytes_read: i64,
    pub bytes_written: i64,
    pub creation_time: f64,
    pub last_message_time: f64,
    pub local_port: i64,
    pub node_id: String,
    pub peer_host: String,
    pub peer_port: i64,
    pub peer_server_port: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RoutesResponse {
    pub routes: Option<Vec<String>>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HealthzResponse {
    pub success: bool,
    pub error: Option<String>,
}
