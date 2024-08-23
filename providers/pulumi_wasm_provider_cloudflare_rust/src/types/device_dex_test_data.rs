#[derive(serde::Serialize)]
pub struct DeviceDexTestData {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
}
