#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigSignIn {
    /// Whether to allow more than one account to have the same email.
    #[builder(into, default)]
    #[serde(rename = "allowDuplicateEmails")]
    pub r#allow_duplicate_emails: Box<Option<bool>>,
    /// Configuration options related to authenticating an anonymous user.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "anonymous")]
    pub r#anonymous: Box<Option<super::super::types::identityplatform::ConfigSignInAnonymous>>,
    /// Configuration options related to authenticating a user by their email address.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<super::super::types::identityplatform::ConfigSignInEmail>>,
    /// (Output)
    /// Output only. Hash config information.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "hashConfigs")]
    pub r#hash_configs: Box<Option<Vec<super::super::types::identityplatform::ConfigSignInHashConfig>>>,
    /// Configuration options related to authenticated a user by their phone number.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<Option<super::super::types::identityplatform::ConfigSignInPhoneNumber>>,
}
