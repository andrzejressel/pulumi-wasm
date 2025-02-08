#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BudgetActionDefinition {
    /// The AWS Identity and Access Management (IAM) action definition details. See IAM Action Definition.
    #[builder(into, default)]
    #[serde(rename = "iamActionDefinition")]
    pub r#iam_action_definition: Box<Option<super::super::types::budgets::BudgetActionDefinitionIamActionDefinition>>,
    /// The service control policies (SCPs) action definition details. See SCP Action Definition.
    #[builder(into, default)]
    #[serde(rename = "scpActionDefinition")]
    pub r#scp_action_definition: Box<Option<super::super::types::budgets::BudgetActionDefinitionScpActionDefinition>>,
    /// The AWS Systems Manager (SSM) action definition details. See SSM Action Definition.
    #[builder(into, default)]
    #[serde(rename = "ssmActionDefinition")]
    pub r#ssm_action_definition: Box<Option<super::super::types::budgets::BudgetActionDefinitionSsmActionDefinition>>,
}
