#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainAdvancedSecurityOptions {
    /// Whether Anonymous auth is enabled. Enables fine-grained access control on an existing domain. Ignored unless `advanced_security_options` are enabled. _Can only be enabled on an existing domain._
    #[builder(into, default)]
    #[serde(rename = "anonymousAuthEnabled")]
    pub r#anonymous_auth_enabled: Box<Option<bool>>,
    /// Whether advanced security is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Whether the internal user database is enabled. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "internalUserDatabaseEnabled")]
    pub r#internal_user_database_enabled: Box<Option<bool>>,
    /// Configuration block for the main user. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "masterUserOptions")]
    pub r#master_user_options: Box<Option<super::super::types::opensearch::DomainAdvancedSecurityOptionsMasterUserOptions>>,
}
