#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterShardNode {
    /// The Availability Zone in which the node resides.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
    /// The date and time when the node was created. Example: `2022-01-01T21:00:00Z`.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    #[builder(into)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Vec<super::super::types::memorydb::GetClusterShardNodeEndpoint>>,
    /// Name of the cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}