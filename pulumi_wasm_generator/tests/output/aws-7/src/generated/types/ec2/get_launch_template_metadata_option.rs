#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplateMetadataOption {
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Box<String>,
    #[builder(into)]
    #[serde(rename = "httpProtocolIpv6")]
    pub r#http_protocol_ipv_6: Box<String>,
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Box<i32>,
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Box<String>,
    #[builder(into)]
    #[serde(rename = "instanceMetadataTags")]
    pub r#instance_metadata_tags: Box<String>,
}
