#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResponsePlanActionSsmAutomation {
    /// The automation document's name.
    #[builder(into)]
    #[serde(rename = "documentName")]
    pub r#document_name: Box<String>,
    /// The version of the automation document to use at runtime.
    #[builder(into, default)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Box<Option<String>>,
    /// The key-value pair to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
    #[builder(into, default)]
    #[serde(rename = "dynamicParameters")]
    pub r#dynamic_parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// The key-value pair parameters to use when the automation document runs. The following values are supported:
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::ssmincidents::ResponsePlanActionSsmAutomationParameter>>>,
    /// The Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The account that the automation document runs in. This can be in either the management account or an application account.
    #[builder(into, default)]
    #[serde(rename = "targetAccount")]
    pub r#target_account: Box<Option<String>>,
}
