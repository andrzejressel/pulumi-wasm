
#[derive(serde::Serialize)]
pub struct Connection {
    #[serde(rename = "agentSocketPath")]
    pub r#agent_socket_path: Box<Option<String>>,
    #[serde(rename = "dialErrorLimit")]
    pub r#dial_error_limit: Box<Option<i32>>,
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "perDialTimeout")]
    pub r#per_dial_timeout: Box<Option<i32>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<f64>>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    #[serde(rename = "privateKeyPassword")]
    pub r#private_key_password: Box<Option<String>>,
    #[serde(rename = "proxy")]
    pub r#proxy: Box<Option<crate::types::ProxyConnection>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ProxyConnection {
    #[serde(rename = "agentSocketPath")]
    pub r#agent_socket_path: Box<Option<String>>,
    #[serde(rename = "dialErrorLimit")]
    pub r#dial_error_limit: Box<Option<i32>>,
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "perDialTimeout")]
    pub r#per_dial_timeout: Box<Option<i32>>,
    #[serde(rename = "port")]
    pub r#port: Box<Option<f64>>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    #[serde(rename = "privateKeyPassword")]
    pub r#private_key_password: Box<Option<String>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}


