#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureGroupOnlineStoreConfig {
    /// Set to `true` to disable the automatic creation of an AWS Glue table when configuring an OfflineStore.
    #[builder(into, default)]
    #[serde(rename = "enableOnlineStore")]
    pub r#enable_online_store: Box<Option<bool>>,
    /// Security config for at-rest encryption of your OnlineStore. See Security Config Below.
    #[builder(into, default)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Box<Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigSecurityConfig>>,
    /// Option for different tiers of low latency storage for real-time data retrieval. Valid values are `Standard`, or `InMemory`.
    #[builder(into, default)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<Option<String>>,
    /// Time to live duration, where the record is hard deleted after the expiration time is reached; ExpiresAt = EventTime + TtlDuration.. See TTl Duration Below.
    #[builder(into, default)]
    #[serde(rename = "ttlDuration")]
    pub r#ttl_duration: Box<Option<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigTtlDuration>>,
}
