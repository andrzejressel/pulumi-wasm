#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReportGroupExportConfig {
    /// contains information about the S3 bucket where the run of a report is exported. see S3 Destination documented below.
    #[builder(into, default)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Box<Option<super::super::types::codebuild::ReportGroupExportConfigS3Destination>>,
    /// The export configuration type. Valid values are `S3` and `NO_EXPORT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
