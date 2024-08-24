#[derive(serde::Serialize)]
pub struct WaitingRoomAdditionalRoute {
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
