#[derive(serde::Serialize)]
pub struct WaitingRoomAdditionalRoute {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
