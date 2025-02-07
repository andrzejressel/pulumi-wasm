#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMesh {
    /// Required. Name of the Kubernetes Deployment whose traffic is managed by the specified HTTPRoute and Service.
    #[builder(into)]
    #[serde(rename = "deployment")]
    pub r#deployment: Box<String>,
    /// Required. Name of the Gateway API HTTPRoute.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Box<String>,
    /// Optional. The label to use when selecting Pods for the Deployment and Service resources. This label must already be present in both resources.
    #[builder(into, default)]
    #[serde(rename = "podSelectorLabel")]
    pub r#pod_selector_label: Box<Option<String>>,
    /// Optional. Route destinations allow configuring the Gateway API HTTPRoute to be deployed to additional clusters. This option is available for multi-cluster service mesh set ups that require the route to exist in the clusters that call the service. If unspecified, the HTTPRoute will only be deployed to the Target cluster.
    #[builder(into, default)]
    #[serde(rename = "routeDestinations")]
    pub r#route_destinations: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMeshRouteDestinations>>,
    /// Optional. The time to wait for route updates to propagate. The maximum configurable time is 3 hours, in seconds format. If unspecified, there is no wait time.
    #[builder(into, default)]
    #[serde(rename = "routeUpdateWaitTime")]
    pub r#route_update_wait_time: Box<Option<String>>,
    /// Required. Name of the Kubernetes Service.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    /// Optional. The amount of time to migrate traffic back from the canary Service to the original Service during the stable phase deployment. If specified, must be between 15s and 3600s. If unspecified, there is no cutback time.
    #[builder(into, default)]
    #[serde(rename = "stableCutbackDuration")]
    pub r#stable_cutback_duration: Box<Option<String>>,
}
