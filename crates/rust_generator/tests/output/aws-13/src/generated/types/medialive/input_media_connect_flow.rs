#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InputMediaConnectFlow {
    /// The ARN of the MediaConnect Flow
    #[builder(into)]
    #[serde(rename = "flowArn")]
    pub r#flow_arn: Box<String>,
}
