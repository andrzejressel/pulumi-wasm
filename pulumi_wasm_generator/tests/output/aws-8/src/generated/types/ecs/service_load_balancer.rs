#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceLoadBalancer {
    /// Name of the container to associate with the load balancer (as it appears in a container definition).
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Box<String>,
    /// Port on the container to associate with the load balancer.
    /// 
    /// > **Version note:** Multiple `load_balancer` configuration block support was added in version 2.22.0 of the provider. This allows configuration of [ECS service support for multiple target groups](https://aws.amazon.com/about-aws/whats-new/2019/07/amazon-ecs-services-now-support-multiple-load-balancer-target-groups/).
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Box<i32>,
    /// Name of the ELB (Classic) to associate with the service.
    #[builder(into, default)]
    #[serde(rename = "elbName")]
    pub r#elb_name: Box<Option<String>>,
    /// ARN of the Load Balancer target group to associate with the service.
    #[builder(into, default)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Box<Option<String>>,
}
