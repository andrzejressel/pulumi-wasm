#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleNrtIncidentGrouping {
    /// A list of alert details to group by, only when the `entity_matching_method` is `Selected`. Possible values are `DisplayName` and `Severity`.
    #[builder(into, default)]
    #[serde(rename = "byAlertDetails")]
    pub r#by_alert_details: Box<Option<Vec<String>>>,
    /// A list of custom details keys to group by, only when the `entity_matching_method` is `Selected`. Only keys defined in the `custom_details` may be used.
    #[builder(into, default)]
    #[serde(rename = "byCustomDetails")]
    pub r#by_custom_details: Box<Option<Vec<String>>>,
    /// A list of entity types to group by, only when the `entity_matching_method` is `Selected`. Possible values are `Account`, `AzureResource`, `CloudApplication`, `DNS`, `File`, `FileHash`, `Host`, `IP`, `Mailbox`, `MailCluster`, `MailMessage`, `Malware`, `Process`, `RegistryKey`, `RegistryValue`, `SecurityGroup`, `SubmissionMail`, `URL`.
    #[builder(into, default)]
    #[serde(rename = "byEntities")]
    pub r#by_entities: Box<Option<Vec<String>>>,
    /// Enable grouping incidents created from alerts triggered by this Sentinel NRT Alert Rule. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The method used to group incidents. Possible values are `AnyAlert`, `Selected` and `AllEntities`. Defaults to `AnyAlert`.
    #[builder(into, default)]
    #[serde(rename = "entityMatchingMethod")]
    pub r#entity_matching_method: Box<Option<String>>,
    /// Limit the group to alerts created within the lookback duration (in ISO 8601 duration format). Defaults to `PT5M`.
    #[builder(into, default)]
    #[serde(rename = "lookbackDuration")]
    pub r#lookback_duration: Box<Option<String>>,
    /// Whether to re-open closed matching incidents? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "reopenClosedIncidents")]
    pub r#reopen_closed_incidents: Box<Option<bool>>,
}
