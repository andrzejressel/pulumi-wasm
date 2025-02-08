#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AssessmentScope {
    /// Amazon Web Services accounts that are in scope for the assessment. See `aws_accounts` below.
    #[builder(into, default)]
    #[serde(rename = "awsAccounts")]
    pub r#aws_accounts: Box<Option<Vec<super::super::types::auditmanager::AssessmentScopeAwsAccount>>>,
    /// Amazon Web Services services that are included in the scope of the assessment. See `aws_services` below.
    #[builder(into, default)]
    #[serde(rename = "awsServices")]
    pub r#aws_services: Box<Option<Vec<super::super::types::auditmanager::AssessmentScopeAwsService>>>,
}
