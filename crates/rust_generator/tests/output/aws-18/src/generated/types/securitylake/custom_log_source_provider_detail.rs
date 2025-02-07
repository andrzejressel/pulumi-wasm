#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomLogSourceProviderDetail {
    /// The location of the partition in the Amazon S3 bucket for Security Lake.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The ARN of the IAM role to be used by the entity putting logs into your custom source partition.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
