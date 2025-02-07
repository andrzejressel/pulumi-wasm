#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityBaselineConfigStatisticsResource {
    /// The Amazon S3 URI for the statistics resource.
    #[builder(into, default)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<Option<String>>,
}
