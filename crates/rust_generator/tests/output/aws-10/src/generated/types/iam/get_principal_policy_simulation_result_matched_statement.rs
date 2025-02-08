#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPrincipalPolicySimulationResultMatchedStatement {
    /// Identifier of one of the policies used as input to the simulation.
    #[builder(into)]
    #[serde(rename = "sourcePolicyId")]
    pub r#source_policy_id: Box<String>,
    /// The type of the policy identified in source_policy_id.
    #[builder(into)]
    #[serde(rename = "sourcePolicyType")]
    pub r#source_policy_type: Box<String>,
}
