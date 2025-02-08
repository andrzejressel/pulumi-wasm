#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork {
    /// The [RFC 3339](https://tools.ietf.org/html/rfc3339)
    /// formatted date time string indicating when this whitelist expires.
    #[builder(into, default)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Box<Option<String>>,
    /// A name for this whitelist entry.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A CIDR notation IPv4 or IPv6 address that is allowed to
    /// access this instance. Must be set even if other two attributes are not for
    /// the whitelist to become active.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
