
#[derive(serde::Serialize)]
pub struct Connection {
    pub r#agentSocketPath: Option<String>,
    pub r#dialErrorLimit: Option<i32>,
    pub r#host: String,
    pub r#password: Option<String>,
    pub r#perDialTimeout: Option<i32>,
    pub r#port: Option<f64>,
    pub r#privateKey: Option<String>,
    pub r#privateKeyPassword: Option<String>,
    pub r#proxy: Option<crate::types::ProxyConnection>,
    pub r#user: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ProxyConnection {
    pub r#agentSocketPath: Option<String>,
    pub r#dialErrorLimit: Option<i32>,
    pub r#host: String,
    pub r#password: Option<String>,
    pub r#perDialTimeout: Option<i32>,
    pub r#port: Option<f64>,
    pub r#privateKey: Option<String>,
    pub r#privateKeyPassword: Option<String>,
    pub r#user: Option<String>,
}


