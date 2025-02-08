#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterBrokerNodeGroupInfoStorageInfo {
    /// A block that contains EBS volume information. See below.
    #[builder(into, default)]
    #[serde(rename = "ebsStorageInfo")]
    pub r#ebs_storage_info: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo>>,
}
