#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceMetadataOption {
    /// State of the metadata service: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Box<String>,
    /// Whether the IPv6 endpoint for the instance metadata service is `enabled` or `disabled`
    #[builder(into)]
    #[serde(rename = "httpProtocolIpv6")]
    pub r#http_protocol_ipv_6: Box<String>,
    /// Desired HTTP PUT response hop limit for instance metadata requests.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<i32>,
    /// If session tokens are required: `optional`, `required`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<String>,
    /// If access to instance tags is allowed from the metadata service: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "instanceMetadataTags")]
    pub r#instance_metadata_tags: Box<String>,
}
