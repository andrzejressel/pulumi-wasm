#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseThreatDetectionPolicy {
    /// Specifies a list of alerts which should be disabled. Possible values include `Access_Anomaly`, `Sql_Injection` and `Sql_Injection_Vulnerability`.
    #[builder(into, default)]
    #[serde(rename = "disabledAlerts")]
    pub r#disabled_alerts: Box<Option<Vec<String>>>,
    /// Should the account administrators be emailed when this alert is triggered? Possible values are `Enabled` or `Disabled`. Defaults to `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "emailAccountAdmins")]
    pub r#email_account_admins: Box<Option<String>>,
    /// A list of email addresses which alerts should be sent to.
    #[builder(into, default)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    /// Specifies the number of days to keep in the Threat Detection audit logs.
    #[builder(into, default)]
    #[serde(rename = "retentionDays")]
    pub r#retention_days: Box<Option<i32>>,
    /// The State of the Policy. Possible values are `Enabled` or `Disabled`. Defaults to `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Specifies the identifier key of the Threat Detection audit storage account. Required if `state` is `Enabled`.
    #[builder(into, default)]
    #[serde(rename = "storageAccountAccessKey")]
    pub r#storage_account_access_key: Box<Option<String>>,
    /// Specifies the blob storage endpoint (e.g. <https://example.blob.core.windows.net>). This blob storage will hold all Threat Detection audit logs. Required if `state` is `Enabled`.
    #[builder(into, default)]
    #[serde(rename = "storageEndpoint")]
    pub r#storage_endpoint: Box<Option<String>>,
}
