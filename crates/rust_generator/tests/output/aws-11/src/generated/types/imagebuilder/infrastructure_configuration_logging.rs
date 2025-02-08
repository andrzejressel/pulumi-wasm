#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InfrastructureConfigurationLogging {
    /// Configuration block with S3 logging settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Box<super::super::types::imagebuilder::InfrastructureConfigurationLoggingS3Logs>,
}
