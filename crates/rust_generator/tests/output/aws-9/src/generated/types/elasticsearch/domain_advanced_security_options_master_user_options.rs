#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainAdvancedSecurityOptionsMasterUserOptions {
    /// ARN for the main user. Only specify if `internal_user_database_enabled` is not set or set to `false`.
    #[builder(into, default)]
    #[serde(rename = "masterUserArn")]
    pub r#master_user_arn: Box<Option<String>>,
    /// Main user's username, which is stored in the Amazon Elasticsearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Box<Option<String>>,
    /// Main user's password, which is stored in the Amazon Elasticsearch Service domain's internal database. Only specify if `internal_user_database_enabled` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "masterUserPassword")]
    pub r#master_user_password: Box<Option<String>>,
}
