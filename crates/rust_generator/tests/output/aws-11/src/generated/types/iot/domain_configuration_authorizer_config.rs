#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainConfigurationAuthorizerConfig {
    /// A Boolean that specifies whether the domain configuration's authorization service can be overridden.
    #[builder(into, default)]
    #[serde(rename = "allowAuthorizerOverride")]
    pub r#allow_authorizer_override: Box<Option<bool>>,
    /// The name of the authorization service for a domain configuration.
    #[builder(into, default)]
    #[serde(rename = "defaultAuthorizerName")]
    pub r#default_authorizer_name: Box<Option<String>>,
}
