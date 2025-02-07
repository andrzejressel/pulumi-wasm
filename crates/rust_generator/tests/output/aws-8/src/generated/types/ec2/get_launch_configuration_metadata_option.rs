#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchConfigurationMetadataOption {
    /// State of the metadata service: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Box<String>,
    /// The desired HTTP PUT response hop limit for instance metadata requests.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<i32>,
    /// If session tokens are required: `optional`, `required`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<String>,
}
