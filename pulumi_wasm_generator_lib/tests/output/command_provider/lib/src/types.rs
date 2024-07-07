
#[derive(serde::Serialize)]
pub struct Connection {
    #[serde(rename = "agentSocketPath")]
    pub r#agent_socket_path: Option<String>,
    #[serde(rename = "dialErrorLimit")]
    pub r#dial_error_limit: Option<i32>,
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "perDialTimeout")]
    pub r#per_dial_timeout: Option<i32>,
    #[serde(rename = "port")]
    pub r#port: Option<f64>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    #[serde(rename = "privateKeyPassword")]
    pub r#private_key_password: Option<String>,
    #[serde(rename = "proxy")]
    pub r#proxy: Option<crate::types::ProxyConnection>,
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ProxyConnection {
    #[serde(rename = "agentSocketPath")]
    pub r#agent_socket_path: Option<String>,
    #[serde(rename = "dialErrorLimit")]
    pub r#dial_error_limit: Option<i32>,
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "perDialTimeout")]
    pub r#per_dial_timeout: Option<i32>,
    #[serde(rename = "port")]
    pub r#port: Option<f64>,
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    #[serde(rename = "privateKeyPassword")]
    pub r#private_key_password: Option<String>,
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}


