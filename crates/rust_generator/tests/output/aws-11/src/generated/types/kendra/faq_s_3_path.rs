#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FaqS3Path {
    /// The name of the S3 bucket that contains the file.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// The name of the file.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
