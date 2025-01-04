#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelDestinationSetting {
    /// Key used to extract the password from EC2 Parameter store.
    #[builder(into, default)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Box<Option<String>>,
    /// Stream name RTMP destinations (URLs of type rtmp://)
    #[builder(into, default)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<Option<String>>,
    /// A URL specifying a destination.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// Username for destination.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
