#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSetRowLevelPermissionTagConfiguration {
    /// The status of row-level security tags. If enabled, the status is `ENABLED`. If disabled, the status is `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// A set of rules associated with row-level security, such as the tag names and columns that they are assigned to. See tag_rules.
    #[builder(into)]
    #[serde(rename = "tagRules")]
    pub r#tag_rules: Box<Vec<super::super::types::quicksight::DataSetRowLevelPermissionTagConfigurationTagRule>>,
}
