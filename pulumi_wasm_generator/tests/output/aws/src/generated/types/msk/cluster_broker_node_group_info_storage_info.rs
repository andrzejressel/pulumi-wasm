#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterBrokerNodeGroupInfoStorageInfo {
    /// A block that contains EBS volume information. See below.
    #[builder(into, default)]
    #[serde(rename = "ebsStorageInfo")]
    pub r#ebs_storage_info: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo>>,
}