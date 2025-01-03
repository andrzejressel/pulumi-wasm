#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigActionExportData {
    /// Store all table and column profiles in an existing table or a new table in an existing dataset. Each re-generation will result in a new row in BigQuery
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "profileTable")]
    pub r#profile_table: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigActionExportDataProfileTable>>,
}
