#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigClientPermissions {
    /// When true, end users cannot delete their account on the associated project through any of our API methods
    #[builder(into, default)]
    #[serde(rename = "disabledUserDeletion")]
    pub r#disabled_user_deletion: Box<Option<bool>>,
    /// When true, end users cannot sign up for a new account on the associated project through any of our API methods
    #[builder(into, default)]
    #[serde(rename = "disabledUserSignup")]
    pub r#disabled_user_signup: Box<Option<bool>>,
}
