#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationDefinition {
    /// JSON application definition. Either this or `s3_location` must be specified.
    #[builder(into, default)]
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Location of the application definition in S3. Either this or `content` must be specified.
    #[builder(into, default)]
    #[serde(rename = "s3Location")]
    pub r#s_3_location: Box<Option<String>>,
}
