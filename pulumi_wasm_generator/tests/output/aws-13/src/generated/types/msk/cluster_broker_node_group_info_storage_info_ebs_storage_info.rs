#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
    /// A block that contains EBS volume provisioned throughput information. To provision storage throughput, you must choose broker type kafka.m5.4xlarge or larger. See below.
    #[builder(into, default)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput>>,
    /// The size in GiB of the EBS volume for the data drive on each broker node. Minimum value of `1` and maximum value of `16384`.
    #[builder(into, default)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<Option<i32>>,
}
