#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProtectionContainerMappingAutomaticUpdate {
    /// The authentication type used for automation account. Possible values are `RunAsAccount` and `SystemAssignedIdentity`. Defaults to `SystemAssignedIdentity`.
    /// 
    /// > **Note:** `RunAsAccount` of `authentication_type` is deprecated and will retire on September 30, 2023. Details could be found [here](https://learn.microsoft.com/en-us/azure/automation/whats-new#support-for-run-as-accounts).
    #[builder(into, default)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: Box<Option<String>>,
    /// The automation account ID which holds the automatic update runbook and authenticates to Azure resources.
    /// 
    /// > **Note:** `automation_account_id` is required when `enabled` is specified.
    #[builder(into, default)]
    #[serde(rename = "automationAccountId")]
    pub r#automation_account_id: Box<Option<String>>,
    /// Should the Mobility service installed on Azure virtual machines be automatically updated. Defaults to `false`.
    /// 
    /// > **Note:** The setting applies to all Azure VMs protected in the same container. For more details see [this document](https://learn.microsoft.com/en-us/azure/site-recovery/azure-to-azure-autoupdate#enable-automatic-updates)
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
