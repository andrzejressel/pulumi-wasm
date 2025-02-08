#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AliasRoutingConfiguration {
    /// The Amazon Resource Name (ARN) of the state machine version.
    #[builder(into)]
    #[serde(rename = "stateMachineVersionArn")]
    pub r#state_machine_version_arn: Box<String>,
    /// Percentage of traffic routed to the state machine version.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
