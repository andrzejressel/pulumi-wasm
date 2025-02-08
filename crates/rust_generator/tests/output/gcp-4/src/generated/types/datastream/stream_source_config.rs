#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamSourceConfig {
    /// MySQL data source configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "mysqlSourceConfig")]
    pub r#mysql_source_config: Box<Option<super::super::types::datastream::StreamSourceConfigMysqlSourceConfig>>,
    /// MySQL data source configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oracleSourceConfig")]
    pub r#oracle_source_config: Box<Option<super::super::types::datastream::StreamSourceConfigOracleSourceConfig>>,
    /// PostgreSQL data source configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "postgresqlSourceConfig")]
    pub r#postgresql_source_config: Box<Option<super::super::types::datastream::StreamSourceConfigPostgresqlSourceConfig>>,
    /// Source connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}
    #[builder(into)]
    #[serde(rename = "sourceConnectionProfile")]
    pub r#source_connection_profile: Box<String>,
    /// SQL Server data source configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sqlServerSourceConfig")]
    pub r#sql_server_source_config: Box<Option<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfig>>,
}
