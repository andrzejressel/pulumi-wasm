#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GroupPolicyAssignmentNonComplianceMessage {
    /// The non-compliance message text. When assigning policy sets (initiatives), unless `policy_definition_reference_id` is specified then this message will be the default for all policies.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// When assigning policy sets (initiatives), this is the ID of the policy definition that the non-compliance message applies to.
    #[builder(into, default)]
    #[serde(rename = "policyDefinitionReferenceId")]
    pub r#policy_definition_reference_id: Box<Option<String>>,
}
