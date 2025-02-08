#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecurityProfileThreatPreventionProfileThreatOverride {
    /// Threat action.
    /// Possible values are: `ALERT`, `ALLOW`, `DEFAULT_ACTION`, `DENY`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Vendor-specific ID of a threat to override.
    #[builder(into)]
    #[serde(rename = "threatId")]
    pub r#threat_id: Box<String>,
    /// (Output)
    /// Type of threat.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
