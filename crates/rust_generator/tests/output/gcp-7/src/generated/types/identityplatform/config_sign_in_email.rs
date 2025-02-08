#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigSignInEmail {
    /// Whether email auth is enabled for the project or not.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Whether a password is required for email auth or not. If true, both an email and
    /// password must be provided to sign in. If false, a user may sign in via either
    /// email/password or email link.
    #[builder(into, default)]
    #[serde(rename = "passwordRequired")]
    pub r#password_required: Box<Option<bool>>,
}
