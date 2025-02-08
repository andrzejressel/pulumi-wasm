#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutonomousDatabasePropertiesConnectionString {
    /// A list of all connection strings that can be used to connect to the
    /// Autonomous Database.
    #[builder(into, default)]
    #[serde(rename = "allConnectionStrings")]
    pub r#all_connection_strings: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesConnectionStringAllConnectionString>>>,
    /// The database service provides the least level of resources to each SQL
    /// statement, but supports the most number of concurrent SQL statements.
    #[builder(into, default)]
    #[serde(rename = "dedicated")]
    pub r#dedicated: Box<Option<String>>,
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
    /// A list of connection string profiles to allow clients to group, filter, and
    /// select values based on the structured metadata.
    #[builder(into, default)]
    #[serde(rename = "profiles")]
    pub r#profiles: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesConnectionStringProfile>>>,
}
