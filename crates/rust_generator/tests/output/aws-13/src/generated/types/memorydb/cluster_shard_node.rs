#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterShardNode {
    /// The Availability Zone in which the node resides.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// The date and time when the node was created. Example: `2022-01-01T21:00:00Z`.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Option<Vec<super::super::types::memorydb::ClusterShardNodeEndpoint>>>,
    /// Name of the cluster. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
