#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAutonomousDatabasePropertyConnectionString {
    /// A list of all connection strings that can be used to connect to the
    /// Autonomous Database.
    #[builder(into)]
    #[serde(rename = "allConnectionStrings")]
    pub r#all_connection_strings: Box<Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyConnectionStringAllConnectionString>>,
    /// The database service provides the least level of resources to each SQL
    /// statement, but supports the most number of concurrent SQL statements.
    #[builder(into)]
    #[serde(rename = "dedicated")]
    pub r#dedicated: Box<String>,
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
    /// A list of connection string profiles to allow clients to group, filter, and
    /// select values based on the structured metadata.
    #[builder(into)]
    #[serde(rename = "profiles")]
    pub r#profiles: Box<Vec<super::super::types::oracledatabase::GetAutonomousDatabasePropertyConnectionStringProfile>>,
}
