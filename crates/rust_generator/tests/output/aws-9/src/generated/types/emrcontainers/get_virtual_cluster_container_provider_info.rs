#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualClusterContainerProviderInfo {
    /// Nested list containing EKS-specific information about the cluster where the EMR Containers cluster is running
    #[builder(into)]
    #[serde(rename = "eksInfos")]
    pub r#eks_infos: Box<Vec<super::super::types::emrcontainers::GetVirtualClusterContainerProviderInfoEksInfo>>,
}
