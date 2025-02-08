#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigClient {
    /// (Output)
    /// API key that can be used when making requests for this project.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<Option<String>>,
    /// (Output)
    /// Firebase subdomain.
    #[builder(into, default)]
    #[serde(rename = "firebaseSubdomain")]
    pub r#firebase_subdomain: Box<Option<String>>,
    /// Configuration related to restricting a user's ability to affect their account.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<super::super::types::identityplatform::ConfigClientPermissions>>,
}
