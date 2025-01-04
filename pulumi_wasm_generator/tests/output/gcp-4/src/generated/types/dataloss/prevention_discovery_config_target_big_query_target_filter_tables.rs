#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilterTables {
    /// A collection of regular expressions to match a BQ table against.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includeRegexes")]
    pub r#include_regexes: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetBigQueryTargetFilterTablesIncludeRegexes>>,
}
