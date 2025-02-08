#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HubS3StorageConfig {
    /// The Amazon S3 bucket prefix for hosting hub content.interface.
    #[builder(into, default)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: Box<Option<String>>,
}
