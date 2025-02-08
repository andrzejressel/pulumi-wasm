#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamSourceConfigMysqlSourceConfigExcludeObjectsMysqlDatabase {
    /// Database name.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// Tables in the database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mysqlTables")]
    pub r#mysql_tables: Box<Option<Vec<super::super::types::datastream::StreamSourceConfigMysqlSourceConfigExcludeObjectsMysqlDatabaseMysqlTable>>>,
}
