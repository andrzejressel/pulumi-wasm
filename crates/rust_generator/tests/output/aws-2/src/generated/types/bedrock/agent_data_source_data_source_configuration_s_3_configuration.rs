#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceDataSourceConfigurationS3Configuration {
    /// ARN of the bucket that contains the data source.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    /// Bucket account owner ID for the S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "bucketOwnerAccountId")]
    pub r#bucket_owner_account_id: Box<Option<String>>,
    /// List of S3 prefixes that define the object containing the data sources. For more information, see [Organizing objects using prefixes](https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-prefixes.html).
    #[builder(into, default)]
    #[serde(rename = "inclusionPrefixes")]
    pub r#inclusion_prefixes: Box<Option<Vec<String>>>,
}
