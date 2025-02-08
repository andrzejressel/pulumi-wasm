#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZoneSoaRecord {
    /// The email contact for the SOA record.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// The expire time for the SOA record. Defaults to `2419200`.
    #[builder(into, default)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// The domain name of the authoritative name server for the SOA record. If not set, computed value from Azure will be used.
    #[builder(into, default)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<Option<String>>,
    /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration. Defaults to `300`.
    #[builder(into, default)]
    #[serde(rename = "minimumTtl")]
    pub r#minimum_ttl: Box<Option<i32>>,
    /// The refresh time for the SOA record. Defaults to `3600`.
    #[builder(into, default)]
    #[serde(rename = "refreshTime")]
    pub r#refresh_time: Box<Option<i32>>,
    /// The retry time for the SOA record. Defaults to `300`.
    #[builder(into, default)]
    #[serde(rename = "retryTime")]
    pub r#retry_time: Box<Option<i32>>,
    /// The serial number for the SOA record. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<i32>>,
    /// A mapping of tags to assign to the Record Set.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Time To Live of the SOA Record in seconds. Defaults to `3600`.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}
