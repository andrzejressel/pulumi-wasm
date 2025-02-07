#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceGroupManagerStatusAllInstancesConfig {
    /// Current all-instances configuration revision. This value is in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "currentRevision")]
    pub r#current_revision: Box<String>,
    /// A bit indicating whether this configuration has been applied to all managed instances in the group.
    #[builder(into)]
    #[serde(rename = "effective")]
    pub r#effective: Box<bool>,
}
