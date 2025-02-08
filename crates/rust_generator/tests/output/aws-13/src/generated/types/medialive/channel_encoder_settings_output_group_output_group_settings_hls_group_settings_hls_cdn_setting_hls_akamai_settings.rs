#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings {
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into, default)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "httpTransferMode")]
    pub r#http_transfer_mode: Box<Option<String>>,
    /// Number of retry attempts.
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into, default)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "salt")]
    pub r#salt: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "token")]
    pub r#token: Box<Option<String>>,
}
