#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrganizationRoot {
    /// ARN of the root
    #[builder(into, default)]
    #[serde(rename = "arn")]
    pub r#arn: Box<Option<String>>,
    /// Identifier of the root
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the policy type
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// List of policy types enabled for this root. All elements have these attributes:
    #[builder(into, default)]
    #[serde(rename = "policyTypes")]
    pub r#policy_types: Box<Option<Vec<super::super::types::organizations::OrganizationRootPolicyType>>>,
}
