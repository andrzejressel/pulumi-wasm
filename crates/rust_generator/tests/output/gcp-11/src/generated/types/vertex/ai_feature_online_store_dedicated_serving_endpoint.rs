#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureOnlineStoreDedicatedServingEndpoint {
    /// Private service connect config.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "privateServiceConnectConfig")]
    pub r#private_service_connect_config: Box<Option<super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpointPrivateServiceConnectConfig>>,
    /// (Output)
    /// Domain name to use for this FeatureOnlineStore
    #[builder(into, default)]
    #[serde(rename = "publicEndpointDomainName")]
    pub r#public_endpoint_domain_name: Box<Option<String>>,
    /// (Output)
    /// Name of the service attachment resource. Applicable only if private service connect is enabled and after FeatureViewSync is created.
    #[builder(into, default)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Box<Option<String>>,
}
