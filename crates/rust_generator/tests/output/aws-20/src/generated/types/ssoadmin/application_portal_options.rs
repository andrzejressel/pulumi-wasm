#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationPortalOptions {
    /// Sign-in options for the access portal. See `sign_in_options` below.
    #[builder(into, default)]
    #[serde(rename = "signInOptions")]
    pub r#sign_in_options: Box<Option<super::super::types::ssoadmin::ApplicationPortalOptionsSignInOptions>>,
    /// Indicates whether this application is visible in the access portal. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "visibility")]
    pub r#visibility: Box<Option<String>>,
}
