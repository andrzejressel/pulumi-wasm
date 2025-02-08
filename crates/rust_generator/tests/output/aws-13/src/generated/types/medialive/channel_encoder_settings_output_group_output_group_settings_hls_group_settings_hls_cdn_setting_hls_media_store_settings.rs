#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings {
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into, default)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "mediaStoreStorageClass")]
    pub r#media_store_storage_class: Box<Option<String>>,
    /// Number of retry attempts.
    #[builder(into, default)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Box<Option<i32>>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into, default)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Box<Option<i32>>,
}
