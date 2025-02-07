#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrganizationPolicyListPolicy {
    /// One or the other must be set.
    #[builder(into)]
    #[serde(rename = "allows")]
    pub r#allows: Box<Vec<super::super::types::projects::GetOrganizationPolicyListPolicyAllow>>,
    /// One or the other must be set.
    #[builder(into)]
    #[serde(rename = "denies")]
    pub r#denies: Box<Vec<super::super::types::projects::GetOrganizationPolicyListPolicyDeny>>,
    /// If set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy.
    #[builder(into)]
    #[serde(rename = "inheritFromParent")]
    pub r#inherit_from_parent: Box<bool>,
    /// The Google Cloud Console will try to default to a configuration that matches the value specified in this field.
    #[builder(into)]
    #[serde(rename = "suggestedValue")]
    pub r#suggested_value: Box<String>,
}
