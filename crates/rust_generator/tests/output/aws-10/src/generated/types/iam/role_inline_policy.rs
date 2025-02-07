#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoleInlinePolicy {
    /// Name of the role policy.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Policy document as a JSON formatted string.
    #[builder(into, default)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<String>>,
}
