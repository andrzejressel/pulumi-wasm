#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PosturePolicySet {
    /// Description of the policy set.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// List of security policy
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policies")]
    pub r#policies: Box<Vec<super::super::types::securityposture::PosturePolicySetPolicy>>,
    /// ID of the policy set.
    #[builder(into)]
    #[serde(rename = "policySetId")]
    pub r#policy_set_id: Box<String>,
}
