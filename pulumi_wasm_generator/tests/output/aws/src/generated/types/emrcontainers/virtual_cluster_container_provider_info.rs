#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualClusterContainerProviderInfo {
    /// Nested list containing EKS-specific information about the cluster where the EMR Containers cluster is running
    #[builder(into)]
    #[serde(rename = "eksInfo")]
    pub r#eks_info: Box<super::super::types::emrcontainers::VirtualClusterContainerProviderInfoEksInfo>,
}
