#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPolicyAssignmentNonComplianceMessage {
    /// The non-compliance message text.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// The ID of the Policy Definition that the non-compliance message applies to.
    #[builder(into)]
    #[serde(rename = "policyDefinitionReferenceId")]
    pub r#policy_definition_reference_id: Box<String>,
}
