#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainSamlOptionsSamlOptions {
    /// Whether SAML authentication is enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Information from your identity provider.
    #[builder(into, default)]
    #[serde(rename = "idp")]
    pub r#idp: Box<Option<super::super::types::elasticsearch::DomainSamlOptionsSamlOptionsIdp>>,
    /// This backend role from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into, default)]
    #[serde(rename = "masterBackendRole")]
    pub r#master_backend_role: Box<Option<String>>,
    /// This username from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into, default)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Box<Option<String>>,
    /// Element of the SAML assertion to use for backend roles. Default is roles.
    #[builder(into, default)]
    #[serde(rename = "rolesKey")]
    pub r#roles_key: Box<Option<String>>,
    /// Duration of a session in minutes after a user logs in. Default is 60. Maximum value is 1,440.
    #[builder(into, default)]
    #[serde(rename = "sessionTimeoutMinutes")]
    pub r#session_timeout_minutes: Box<Option<i32>>,
    /// Custom SAML attribute to use for user names. Default is an empty string - `""`. This will cause Elasticsearch to use the `NameID` element of the `Subject`, which is the default location for name identifiers in the SAML specification.
    #[builder(into, default)]
    #[serde(rename = "subjectKey")]
    pub r#subject_key: Box<Option<String>>,
}
