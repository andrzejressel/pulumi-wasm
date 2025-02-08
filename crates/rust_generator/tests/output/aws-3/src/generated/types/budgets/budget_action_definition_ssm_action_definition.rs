#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BudgetActionDefinitionSsmActionDefinition {
    /// The action subType. Valid values are `STOP_EC2_INSTANCES` or `STOP_RDS_INSTANCES`.
    #[builder(into)]
    #[serde(rename = "actionSubType")]
    pub r#action_sub_type: Box<String>,
    /// The EC2 and RDS instance IDs.
    #[builder(into)]
    #[serde(rename = "instanceIds")]
    pub r#instance_ids: Box<Vec<String>>,
    /// The Region to run the SSM document.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
