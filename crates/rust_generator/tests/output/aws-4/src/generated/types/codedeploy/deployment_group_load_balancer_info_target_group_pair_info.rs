#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeploymentGroupLoadBalancerInfoTargetGroupPairInfo {
    /// Configuration block for the production traffic route (documented below).
    #[builder(into)]
    #[serde(rename = "prodTrafficRoute")]
    pub r#prod_traffic_route: Box<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoProdTrafficRoute>,
    /// Configuration blocks for a target group within a target group pair (documented below).
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Vec<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTargetGroup>>,
    /// Configuration block for the test traffic route (documented below).
    #[builder(into, default)]
    #[serde(rename = "testTrafficRoute")]
    pub r#test_traffic_route: Box<Option<super::super::types::codedeploy::DeploymentGroupLoadBalancerInfoTargetGroupPairInfoTestTrafficRoute>>,
}
