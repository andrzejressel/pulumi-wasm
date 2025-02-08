#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionJobTriggerInspectJobStorageConfigHybridOptions {
    /// A short description of where the data is coming from. Will be stored once in the job. 256 max length.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// To organize findings, these labels will be added to each finding.
    /// Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z?`.
    /// Label values must be between 0 and 63 characters long and must conform to the regular expression `(a-z?)?`.
    /// No more than 10 labels can be associated with a given finding.
    /// Examples:
    /// * `"environment" : "production"`
    /// * `"pipeline" : "etl"`
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// These are labels that each inspection request must include within their 'finding_labels' map. Request
    /// may contain others, but any missing one of these will be rejected.
    /// Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `a-z?`.
    /// No more than 10 keys can be required.
    #[builder(into, default)]
    #[serde(rename = "requiredFindingLabelKeys")]
    pub r#required_finding_label_keys: Box<Option<Vec<String>>>,
    /// If the container is a table, additional information to make findings meaningful such as the columns that are primary keys.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tableOptions")]
    pub r#table_options: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigHybridOptionsTableOptions>>,
}
