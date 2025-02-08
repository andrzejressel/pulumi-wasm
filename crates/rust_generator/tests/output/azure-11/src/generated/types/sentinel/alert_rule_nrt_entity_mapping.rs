#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleNrtEntityMapping {
    /// The type of the entity. Possible values are `Account`, `AzureResource`, `CloudApplication`, `DNS`, `File`, `FileHash`, `Host`, `IP`, `Mailbox`, `MailCluster`, `MailMessage`, `Malware`, `Process`, `RegistryKey`, `RegistryValue`, `SecurityGroup`, `SubmissionMail`, `URL`.
    #[builder(into)]
    #[serde(rename = "entityType")]
    pub r#entity_type: Box<String>,
    /// A list of `field_mapping` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "fieldMappings")]
    pub r#field_mappings: Box<Vec<super::super::types::sentinel::AlertRuleNrtEntityMappingFieldMapping>>,
}
