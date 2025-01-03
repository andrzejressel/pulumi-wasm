#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentGroupLoadBalancerInfo {
    /// The Classic Elastic Load Balancer to use in a deployment. Conflicts with `target_group_info` and `target_group_pair_info`.
    #[builder(into, default)]
    #[serde(rename = "elbInfos")]
    pub r#elb_infos: Box<Option<Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoElbInfo>>>,
    /// The (Application/Network Load Balancer) target group to use in a deployment. Conflicts with `elb_info` and `target_group_pair_info`.
    #[builder(into, default)]
    #[serde(rename = "targetGroupInfos")]
    pub r#target_group_infos: Box<Option<Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupInfo>>>,
    /// The (Application/Network Load Balancer) target group pair to use in a deployment. Conflicts with `elb_info` and `target_group_info`.
    #[builder(into, default)]
    #[serde(rename = "targetGroupPairInfo")]
    pub r#target_group_pair_info: Box<Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfo>>,
}
