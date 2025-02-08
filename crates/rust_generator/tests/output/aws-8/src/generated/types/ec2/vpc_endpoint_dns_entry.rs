#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcEndpointDnsEntry {
    /// The DNS name.
    #[builder(into, default)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Box<Option<String>>,
    /// The ID of the private hosted zone.
    #[builder(into, default)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Box<Option<String>>,
}
