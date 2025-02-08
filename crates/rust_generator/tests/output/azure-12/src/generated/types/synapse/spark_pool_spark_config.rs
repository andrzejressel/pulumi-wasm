#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SparkPoolSparkConfig {
    /// The contents of a spark configuration.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// The name of the file where the spark configuration `content` will be stored.
    #[builder(into)]
    #[serde(rename = "filename")]
    pub r#filename: Box<String>,
}
