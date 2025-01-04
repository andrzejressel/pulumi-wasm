#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZoneDiscoverySpec {
    /// Optional. Configuration for CSV data.
    #[builder(into, default)]
    #[serde(rename = "csvOptions")]
    pub r#csv_options: Box<Option<super::super::types::dataplex::ZoneDiscoverySpecCsvOptions>>,
    /// Required. Whether discovery is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Optional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into, default)]
    #[serde(rename = "excludePatterns")]
    pub r#exclude_patterns: Box<Option<Vec<String>>>,
    /// Optional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into, default)]
    #[serde(rename = "includePatterns")]
    pub r#include_patterns: Box<Option<Vec<String>>>,
    /// Optional. Configuration for Json data.
    #[builder(into, default)]
    #[serde(rename = "jsonOptions")]
    pub r#json_options: Box<Option<super::super::types::dataplex::ZoneDiscoverySpecJsonOptions>>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *".
    #[builder(into, default)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<Option<String>>,
}
