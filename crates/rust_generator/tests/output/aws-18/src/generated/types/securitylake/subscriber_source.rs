#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubscriberSource {
    /// Amazon Security Lake supports log and event collection for natively supported AWS services. See `aws_log_source_resource` Block below.
    #[builder(into, default)]
    #[serde(rename = "awsLogSourceResource")]
    pub r#aws_log_source_resource: Box<Option<super::super::types::securitylake::SubscriberSourceAwsLogSourceResource>>,
    /// Amazon Security Lake supports custom source types. See `custom_log_source_resource` Block below.
    #[builder(into, default)]
    #[serde(rename = "customLogSourceResource")]
    pub r#custom_log_source_resource: Box<Option<super::super::types::securitylake::SubscriberSourceCustomLogSourceResource>>,
}
