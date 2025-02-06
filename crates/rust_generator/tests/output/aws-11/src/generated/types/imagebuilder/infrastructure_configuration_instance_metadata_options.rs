#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InfrastructureConfigurationInstanceMetadataOptions {
    /// The number of hops that an instance can traverse to reach its destonation.
    #[builder(into, default)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<Option<i32>>,
    /// Whether a signed token is required for instance metadata retrieval requests. Valid values: `required`, `optional`.
    #[builder(into, default)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<Option<String>>,
}
