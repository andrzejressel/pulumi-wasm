#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QuerySuggestionsBlockListSourceS3Path {
    /// Name of the S3 bucket that contains the file.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Name of the file.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
}
