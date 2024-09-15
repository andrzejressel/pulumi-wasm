#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomAdditionalRoute {
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
