#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatascanDataProfileSpec {
    /// The fields to exclude from data profile.
    /// If specified, the fields will be excluded from data profile, regardless of `include_fields` value.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeFields")]
    pub r#exclude_fields: Box<Option<super::super::types::dataplex::DatascanDataProfileSpecExcludeFields>>,
    /// The fields to include in data profile.
    /// If not specified, all fields at the time of profile scan job execution are included, except for ones listed in `exclude_fields`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includeFields")]
    pub r#include_fields: Box<Option<super::super::types::dataplex::DatascanDataProfileSpecIncludeFields>>,
    /// Actions to take upon job completion.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postScanActions")]
    pub r#post_scan_actions: Box<Option<super::super::types::dataplex::DatascanDataProfileSpecPostScanActions>>,
    /// A filter applied to all rows in a single DataScan job. The filter needs to be a valid SQL expression for a WHERE clause in BigQuery standard SQL syntax. Example: col1 >= 0 AND col2 < 10
    #[builder(into, default)]
    #[serde(rename = "rowFilter")]
    pub r#row_filter: Box<Option<String>>,
    /// The percentage of the records to be selected from the dataset for DataScan.
    /// Value can range between 0.0 and 100.0 with up to 3 significant decimal digits.
    /// Sampling is not applied if `sampling_percent` is not specified, 0 or 100.
    #[builder(into, default)]
    #[serde(rename = "samplingPercent")]
    pub r#sampling_percent: Box<Option<f64>>,
}
