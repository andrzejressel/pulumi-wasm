#[derive(serde::Serialize)]
pub struct HyperdriveConfigOrigin {
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
    #[serde(rename = "user")]
    pub r#user: Box<String>,
}
