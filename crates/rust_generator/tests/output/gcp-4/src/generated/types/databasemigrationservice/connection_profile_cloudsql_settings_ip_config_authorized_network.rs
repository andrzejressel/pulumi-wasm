#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileCloudsqlSettingsIpConfigAuthorizedNetwork {
    /// The time when this access control entry expires in RFC 3339 format.
    #[builder(into, default)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Box<Option<String>>,
    /// A label to identify this entry.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// Input only. The time-to-leave of this access control entry.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<String>>,
    /// The allowlisted value for the access control list.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
