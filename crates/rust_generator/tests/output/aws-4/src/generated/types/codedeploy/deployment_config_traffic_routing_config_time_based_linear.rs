#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentConfigTrafficRoutingConfigTimeBasedLinear {
    /// The number of minutes between each incremental traffic shift of a `TimeBasedLinear` deployment.
    #[builder(into, default)]
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<i32>>,
    /// The percentage of traffic that is shifted at the start of each increment of a `TimeBasedLinear` deployment.
    #[builder(into, default)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<Option<i32>>,
}
