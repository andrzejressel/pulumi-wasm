#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetes {
    /// Kubernetes Gateway API service mesh configuration.
    #[builder(into, default)]
    #[serde(rename = "gatewayServiceMesh")]
    pub r#gateway_service_mesh: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesGatewayServiceMesh>>,
    /// Kubernetes Service networking configuration.
    #[builder(into, default)]
    #[serde(rename = "serviceNetworking")]
    pub r#service_networking: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryRuntimeConfigKubernetesServiceNetworking>>,
}
