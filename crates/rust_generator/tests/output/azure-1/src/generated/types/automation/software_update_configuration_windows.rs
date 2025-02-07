#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SoftwareUpdateConfigurationWindows {
    /// Specifies the list of update classification. Possible values are `Unclassified`, `Critical`, `Security`, `UpdateRollup`, `FeaturePack`, `ServicePack`, `Definition`, `Tools` and `Updates`.
    /// 
    /// > **NOTE:** The `classifications_included` property will become `Required` in version 4.0 of the Provider.
    #[builder(into)]
    #[serde(rename = "classificationsIncludeds")]
    pub r#classifications_includeds: Box<Vec<String>>,
    /// Specifies a list of knowledge base numbers excluded.
    #[builder(into, default)]
    #[serde(rename = "excludedKnowledgeBaseNumbers")]
    pub r#excluded_knowledge_base_numbers: Box<Option<Vec<String>>>,
    /// Specifies a list of knowledge base numbers included.
    #[builder(into, default)]
    #[serde(rename = "includedKnowledgeBaseNumbers")]
    pub r#included_knowledge_base_numbers: Box<Option<Vec<String>>>,
    /// Specifies the reboot settings after software update, possible values are `IfRequired`, `Never`, `RebootOnly` and `Always`. Defaults to `IfRequired`.
    #[builder(into, default)]
    #[serde(rename = "reboot")]
    pub r#reboot: Box<Option<String>>,
}
