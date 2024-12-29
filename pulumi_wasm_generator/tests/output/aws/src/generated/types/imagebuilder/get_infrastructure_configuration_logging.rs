#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInfrastructureConfigurationLogging {
    /// Nested list of S3 logs settings.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Box<Vec<super::super::types::imagebuilder::GetInfrastructureConfigurationLoggingS3Log>>,
}
