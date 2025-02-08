#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainAdvancedSecurityOptions {
    /// Whether advanced security is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Whether the internal user database is enabled. If not set, defaults to `false` by the AWS API.
    #[builder(into, default)]
    #[serde(rename = "internalUserDatabaseEnabled")]
    pub r#internal_user_database_enabled: Box<Option<bool>>,
    /// Configuration block for the main user. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "masterUserOptions")]
    pub r#master_user_options: Box<Option<super::super::types::elasticsearch::DomainAdvancedSecurityOptionsMasterUserOptions>>,
}
