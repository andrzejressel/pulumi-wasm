#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IdentityPoolRoleAttachmentRoleMappingMappingRule {
    /// The claim name that must be present in the token, for example, "isAdmin" or "paid".
    #[builder(into)]
    #[serde(rename = "claim")]
    pub r#claim: Box<String>,
    /// The match condition that specifies how closely the claim value in the IdP token must match Value.
    #[builder(into)]
    #[serde(rename = "matchType")]
    pub r#match_type: Box<String>,
    /// The role ARN.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// A brief string that the claim must match, for example, "paid" or "yes".
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
