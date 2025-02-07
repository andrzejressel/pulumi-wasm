#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyRotationPolicyAutomatic {
    /// Rotate automatically at a duration after create as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into, default)]
    #[serde(rename = "timeAfterCreation")]
    pub r#time_after_creation: Box<Option<String>>,
    /// Rotate automatically at a duration before expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into, default)]
    #[serde(rename = "timeBeforeExpiry")]
    pub r#time_before_expiry: Box<Option<String>>,
}
