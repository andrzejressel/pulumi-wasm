#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAutonomousDatabasesAutonomousDatabasePropertyConnectionStringAllConnectionString {
    /// The database service provides the highest level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "high")]
    pub r#high: Box<String>,
    /// The database service provides the least level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "low")]
    pub r#low: Box<String>,
    /// The database service provides a lower level of resources to each SQL
    /// statement.
    #[builder(into)]
    #[serde(rename = "medium")]
    pub r#medium: Box<String>,
}
