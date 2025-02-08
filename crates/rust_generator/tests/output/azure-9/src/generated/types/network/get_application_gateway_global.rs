#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetApplicationGatewayGlobal {
    /// Whether Application Gateway's Request buffer is enabled.
    #[builder(into)]
    #[serde(rename = "requestBufferingEnabled")]
    pub r#request_buffering_enabled: Box<bool>,
    /// Whether Application Gateway's Response buffer is enabled.
    #[builder(into)]
    #[serde(rename = "responseBufferingEnabled")]
    pub r#response_buffering_enabled: Box<bool>,
}
