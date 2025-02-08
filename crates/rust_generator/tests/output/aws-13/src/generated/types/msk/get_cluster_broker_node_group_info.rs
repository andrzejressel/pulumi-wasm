#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterBrokerNodeGroupInfo {
    #[builder(into)]
    #[serde(rename = "azDistribution")]
    pub r#az_distribution: Box<String>,
    #[builder(into)]
    #[serde(rename = "clientSubnets")]
    pub r#client_subnets: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "connectivityInfos")]
    pub r#connectivity_infos: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfo>>,
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "storageInfos")]
    pub r#storage_infos: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoStorageInfo>>,
}
