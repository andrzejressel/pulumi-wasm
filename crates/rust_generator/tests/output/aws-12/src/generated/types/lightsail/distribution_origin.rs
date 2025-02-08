#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionOrigin {
    /// The name of the origin resource. Your origin can be an instance with an attached static IP, a bucket, or a load balancer that has at least one instance attached to it.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content.
    #[builder(into, default)]
    #[serde(rename = "protocolPolicy")]
    pub r#protocol_policy: Box<Option<String>>,
    /// The AWS Region name of the origin resource.
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: Box<String>,
    /// The resource type of the origin resource (e.g., Instance).
    #[builder(into, default)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<Option<String>>,
}
