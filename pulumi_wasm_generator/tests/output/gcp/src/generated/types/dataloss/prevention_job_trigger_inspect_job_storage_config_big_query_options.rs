#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobStorageConfigBigQueryOptions {
    /// References to fields excluded from scanning.
    /// This allows you to skip inspection of entire columns which you know have no findings.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludedFields")]
    pub r#excluded_fields: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsExcludedField>>>,
    /// Specifies the BigQuery fields that will be returned with findings.
    /// If not specified, no identifying fields will be returned for findings.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "identifyingFields")]
    pub r#identifying_fields: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsIdentifyingField>>>,
    /// Limit scanning only to these fields.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includedFields")]
    pub r#included_fields: Box<Option<Vec<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsIncludedField>>>,
    /// Max number of rows to scan. If the table has more rows than this value, the rest of the rows are omitted.
    /// If not set, or if set to 0, all rows will be scanned. Only one of rowsLimit and rowsLimitPercent can be
    /// specified. Cannot be used in conjunction with TimespanConfig.
    #[builder(into, default)]
    #[serde(rename = "rowsLimit")]
    pub r#rows_limit: Box<Option<i32>>,
    /// Max percentage of rows to scan. The rest are omitted. The number of rows scanned is rounded down.
    /// Must be between 0 and 100, inclusively. Both 0 and 100 means no limit. Defaults to 0. Only one of
    /// rowsLimit and rowsLimitPercent can be specified. Cannot be used in conjunction with TimespanConfig.
    #[builder(into, default)]
    #[serde(rename = "rowsLimitPercent")]
    pub r#rows_limit_percent: Box<Option<i32>>,
    /// How to sample rows if not all rows are scanned. Meaningful only when used in conjunction with either
    /// rowsLimit or rowsLimitPercent. If not specified, rows are scanned in the order BigQuery reads them.
    /// If TimespanConfig is set, set this to an empty string to avoid using the default value.
    /// Default value is `TOP`.
    /// Possible values are: `TOP`, `RANDOM_START`.
    #[builder(into, default)]
    #[serde(rename = "sampleMethod")]
    pub r#sample_method: Box<Option<String>>,
    /// Set of files to scan.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tableReference")]
    pub r#table_reference: Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference>,
}
