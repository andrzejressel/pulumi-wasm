#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyRotationPolicy {
    /// An `automatic` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "automatic")]
    pub r#automatic: Box<Option<super::super::types::keyvault::KeyRotationPolicyAutomatic>>,
    /// Expire a Key Vault Key after given duration as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into, default)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Box<Option<String>>,
    /// Notify at a given duration before expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into, default)]
    #[serde(rename = "notifyBeforeExpiry")]
    pub r#notify_before_expiry: Box<Option<String>>,
}
