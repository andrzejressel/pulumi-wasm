#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamBackfillAll {
    /// MySQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mysqlExcludedObjects")]
    pub r#mysql_excluded_objects: Box<Option<super::super::types::datastream::StreamBackfillAllMysqlExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oracleExcludedObjects")]
    pub r#oracle_excluded_objects: Box<Option<super::super::types::datastream::StreamBackfillAllOracleExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postgresqlExcludedObjects")]
    pub r#postgresql_excluded_objects: Box<Option<super::super::types::datastream::StreamBackfillAllPostgresqlExcludedObjects>>,
    /// SQL Server data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sqlServerExcludedObjects")]
    pub r#sql_server_excluded_objects: Box<Option<super::super::types::datastream::StreamBackfillAllSqlServerExcludedObjects>>,
}
