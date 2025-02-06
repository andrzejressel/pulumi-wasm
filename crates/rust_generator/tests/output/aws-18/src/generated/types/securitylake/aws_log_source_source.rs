#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsLogSourceSource {
    /// Specify the AWS account information where you want to enable Security Lake.
    /// If not specified, uses all accounts included in the Security Lake.
    #[builder(into, default)]
    #[serde(rename = "accounts")]
    pub r#accounts: Box<Option<Vec<String>>>,
    /// Specify the Regions where you want to enable Security Lake.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Vec<String>>,
    /// The name for a AWS source. This must be a Regionally unique value. Valid values: `ROUTE53`, `VPC_FLOW`, `SH_FINDINGS`, `CLOUD_TRAIL_MGMT`, `LAMBDA_EXECUTION`, `S3_DATA`, `EKS_AUDIT`, `WAF`.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<String>,
    /// The version for a AWS source.
    /// If not specified, the version will be the default.
    /// This must be a Regionally unique value.
    #[builder(into, default)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Box<Option<String>>,
}
