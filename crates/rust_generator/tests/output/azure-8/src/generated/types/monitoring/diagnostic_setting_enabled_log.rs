#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DiagnosticSettingEnabledLog {
    /// The name of a Diagnostic Log Category for this Resource.
    /// 
    /// > **NOTE:** The Log Categories available vary depending on the Resource being used. You may wish to use the `azure.monitoring.getDiagnosticCategories` Data Source or [list of service specific schemas](https://docs.microsoft.com/azure/azure-monitor/platform/resource-logs-schema#service-specific-schemas) to identify which categories are available for a given Resource.
    #[builder(into, default)]
    #[serde(rename = "category")]
    pub r#category: Box<Option<String>>,
    /// The name of a Diagnostic Log Category Group for this Resource.
    /// 
    /// > **NOTE:** Not all resources have category groups available.
    /// 
    /// > **NOTE:** Exactly one of `category` or `category_group` must be specified.
    #[builder(into, default)]
    #[serde(rename = "categoryGroup")]
    pub r#category_group: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::monitoring::DiagnosticSettingEnabledLogRetentionPolicy>>,
}
