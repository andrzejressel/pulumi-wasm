#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LanguageModelInputDataConfig {
    /// IAM role with access to S3 bucket.
    #[builder(into)]
    #[serde(rename = "dataAccessRoleArn")]
    pub r#data_access_role_arn: Box<String>,
    /// S3 URI where training data is located.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
    /// S3 URI where tuning data is located.
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "tuningDataS3Uri")]
    pub r#tuning_data_s_3_uri: Box<Option<String>>,
}
