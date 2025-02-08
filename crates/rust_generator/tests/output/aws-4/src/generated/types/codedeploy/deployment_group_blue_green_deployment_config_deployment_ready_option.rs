#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption {
    /// When to reroute traffic from an original environment to a replacement environment in a blue/green deployment.
    /// * `CONTINUE_DEPLOYMENT`: Register new instances with the load balancer immediately after the new application revision is installed on the instances in the replacement environment.
    /// * `STOP_DEPLOYMENT`: Do not register new instances with load balancer unless traffic is rerouted manually. If traffic is not rerouted manually before the end of the specified wait period, the deployment status is changed to Stopped.
    #[builder(into, default)]
    #[serde(rename = "actionOnTimeout")]
    pub r#action_on_timeout: Box<Option<String>>,
    /// The number of minutes to wait before the status of a blue/green deployment changed to Stopped if rerouting is not started manually. Applies only to the `STOP_DEPLOYMENT` option for `action_on_timeout`.
    #[builder(into, default)]
    #[serde(rename = "waitTimeInMinutes")]
    pub r#wait_time_in_minutes: Box<Option<i32>>,
}
