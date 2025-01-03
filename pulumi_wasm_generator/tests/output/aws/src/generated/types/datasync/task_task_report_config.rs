#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskTaskReportConfig {
    /// Specifies the type of task report you'd like. Valid values: `SUMMARY_ONLY` and `STANDARD`.
    #[builder(into, default)]
    #[serde(rename = "outputType")]
    pub r#output_type: Box<Option<String>>,
    /// Specifies whether you want your task report to include only what went wrong with your transfer or a list of what succeeded and didn't. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    #[builder(into, default)]
    #[serde(rename = "reportLevel")]
    pub r#report_level: Box<Option<String>>,
    /// Configuration block containing the configuration of the reporting level for aspects of your task report. See `report_overrides` below.
    #[builder(into, default)]
    #[serde(rename = "reportOverrides")]
    pub r#report_overrides: Box<Option<super::super::types::datasync::TaskTaskReportConfigReportOverrides>>,
    /// Configuration block containing the configuration for the Amazon S3 bucket where DataSync uploads your task report. See `s3_destination` below.
    #[builder(into)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Box<super::super::types::datasync::TaskTaskReportConfigS3Destination>,
    /// Specifies whether your task report includes the new version of each object transferred into an S3 bucket. This only applies if you enable versioning on your bucket. Keep in mind that setting this to INCLUDE can increase the duration of your task execution. Valid values: `INCLUDE` and `NONE`.
    #[builder(into, default)]
    #[serde(rename = "s3ObjectVersioning")]
    pub r#s_3_object_versioning: Box<Option<String>>,
}
