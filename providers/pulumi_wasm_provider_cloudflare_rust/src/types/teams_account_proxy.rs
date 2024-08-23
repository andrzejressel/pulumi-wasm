#[derive(serde::Serialize)]
pub struct TeamsAccountProxy {
    #[serde(rename = "rootCa")]
    pub r#root_ca: Box<bool>,
    #[serde(rename = "tcp")]
    pub r#tcp: Box<bool>,
    #[serde(rename = "udp")]
    pub r#udp: Box<bool>,
}
