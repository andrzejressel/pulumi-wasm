#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountShareProperties {
    /// A `cors_rule` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Box<Option<Vec<super::super::types::storage::AccountSharePropertiesCorsRule>>>,
    /// A `retention_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::storage::AccountSharePropertiesRetentionPolicy>>,
    /// A `smb` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "smb")]
    pub r#smb: Box<Option<super::super::types::storage::AccountSharePropertiesSmb>>,
}
