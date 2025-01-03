#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutonomousDatabasePropertiesConnectionStringAllConnectionString {
    /// The database service provides the highest level of resources to each SQL
    /// statement.
    #[builder(into, default)]
    #[serde(rename = "high")]
    pub r#high: Box<Option<String>>,
    /// The database service provides the least level of resources to each SQL
    /// statement.
    #[builder(into, default)]
    #[serde(rename = "low")]
    pub r#low: Box<Option<String>>,
    /// The database service provides a lower level of resources to each SQL
    /// statement.
    #[builder(into, default)]
    #[serde(rename = "medium")]
    pub r#medium: Box<Option<String>>,
}
