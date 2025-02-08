#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetResponsePlanActionSsmAutomation {
    /// The automation document's name.
    #[builder(into)]
    #[serde(rename = "documentName")]
    pub r#document_name: Box<String>,
    /// The version of the automation document to use at runtime.
    #[builder(into)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Box<String>,
    /// The key-value pair used to resolve dynamic parameter values when processing a Systems Manager Automation runbook.
    #[builder(into)]
    #[serde(rename = "dynamicParameters")]
    pub r#dynamic_parameters: Box<std::collections::HashMap<String, String>>,
    /// The key-value pair parameters used when the automation document runs. The following values are supported:
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Vec<super::super::types::ssmincidents::GetResponsePlanActionSsmAutomationParameter>>,
    /// The Amazon Resource Name (ARN) of the role that the automation document assumes when it runs commands.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The account that runs the automation document. This can be in either the management account or an application account.
    #[builder(into)]
    #[serde(rename = "targetAccount")]
    pub r#target_account: Box<String>,
}
