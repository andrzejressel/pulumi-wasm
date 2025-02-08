#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceVpcConnectionProperties {
    /// The Amazon Resource Name (ARN) for the VPC connection.
    #[builder(into)]
    #[serde(rename = "vpcConnectionArn")]
    pub r#vpc_connection_arn: Box<String>,
}
