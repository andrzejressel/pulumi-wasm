#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterCacheNode {
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Availability Zone for the cache cluster.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[builder(into)]
    #[serde(rename = "outpostArn")]
    pub r#outpost_arn: Box<String>,
    /// The port number on which each of the cache nodes will
    /// accept connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
