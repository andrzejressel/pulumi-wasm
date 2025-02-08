#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TaskSetLoadBalancer {
    /// The name of the container to associate with the load balancer (as it appears in a container definition).
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// The port on the container to associate with the load balancer. Defaults to `0` if not specified.
    /// 
    /// > **Note:** Specifying multiple `load_balancer` configurations is still not supported by AWS for ECS task set.
    #[builder(into, default)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<Option<i32>>,
    /// The name of the ELB (Classic) to associate with the service.
    #[builder(into, default)]
    #[serde(rename = "loadBalancerName")]
    pub r#load_balancer_name: Box<Option<String>>,
    /// The ARN of the Load Balancer target group to associate with the service.
    #[builder(into, default)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Box<Option<String>>,
}
