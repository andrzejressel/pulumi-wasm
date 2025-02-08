#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZeroTrustAccessApplicationScimConfig {
    /// Attributes for configuring HTTP Basic, OAuth Bearer token, or OAuth 2 authentication schemes for SCIM provisioning to an application.
    #[builder(into, default)]
    #[serde(rename = "authentication")]
    pub r#authentication: Box<Option<super::types::ZeroTrustAccessApplicationScimConfigAuthentication>>,
    /// If false, propagates DELETE requests to the target application for SCIM resources. If true, sets 'active' to false on the SCIM resource. Note: Some targets do not support DELETE operations.
    #[builder(into, default)]
    #[serde(rename = "deactivateOnDelete")]
    pub r#deactivate_on_delete: Box<Option<bool>>,
    /// Whether SCIM provisioning is turned on for this application.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The UID of the IdP to use as the source for SCIM resources to provision to this application.
    #[builder(into)]
    #[serde(rename = "idpUid")]
    pub r#idp_uid: Box<String>,
    /// A list of mappings to apply to SCIM resources before provisioning them in this application. These can transform or filter the resources to be provisioned.
    #[builder(into, default)]
    #[serde(rename = "mappings")]
    pub r#mappings: Box<Option<Vec<super::types::ZeroTrustAccessApplicationScimConfigMapping>>>,
    /// The base URI for the application's SCIM-compatible API.
    #[builder(into)]
    #[serde(rename = "remoteUri")]
    pub r#remote_uri: Box<String>,
}
