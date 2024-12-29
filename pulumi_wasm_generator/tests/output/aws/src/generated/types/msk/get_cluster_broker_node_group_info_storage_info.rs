#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterBrokerNodeGroupInfoStorageInfo {
    #[builder(into)]
    #[serde(rename = "ebsStorageInfos")]
    pub r#ebs_storage_infos: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo>>,
}
