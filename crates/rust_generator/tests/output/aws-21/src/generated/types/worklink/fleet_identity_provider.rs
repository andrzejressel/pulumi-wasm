#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetIdentityProvider {
    /// The SAML metadata document provided by the customer’s identity provider.
    #[builder(into)]
    #[serde(rename = "samlMetadata")]
    pub r#saml_metadata: Box<String>,
    /// The type of identity provider.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
