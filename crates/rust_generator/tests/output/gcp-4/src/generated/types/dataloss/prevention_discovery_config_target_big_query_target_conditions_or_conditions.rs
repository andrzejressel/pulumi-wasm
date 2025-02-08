#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetConditionsOrConditions {
    /// Duration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1.
    #[builder(into, default)]
    #[serde(rename = "minAge")]
    pub r#min_age: Box<Option<String>>,
    /// Minimum number of rows that should be present before Cloud DLP profiles as a table.
    #[builder(into, default)]
    #[serde(rename = "minRowCount")]
    pub r#min_row_count: Box<Option<i32>>,
}
