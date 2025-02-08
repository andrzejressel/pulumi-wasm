#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParameters {
    /// Parameters for connecting to Amazon Elasticsearch.
    #[builder(into, default)]
    #[serde(rename = "amazonElasticsearch")]
    pub r#amazon_elasticsearch: Box<Option<super::super::types::quicksight::DataSourceParametersAmazonElasticsearch>>,
    /// Parameters for connecting to Athena.
    #[builder(into, default)]
    #[serde(rename = "athena")]
    pub r#athena: Box<Option<super::super::types::quicksight::DataSourceParametersAthena>>,
    /// Parameters for connecting to Aurora MySQL.
    #[builder(into, default)]
    #[serde(rename = "aurora")]
    pub r#aurora: Box<Option<super::super::types::quicksight::DataSourceParametersAurora>>,
    /// Parameters for connecting to Aurora Postgresql.
    #[builder(into, default)]
    #[serde(rename = "auroraPostgresql")]
    pub r#aurora_postgresql: Box<Option<super::super::types::quicksight::DataSourceParametersAuroraPostgresql>>,
    /// Parameters for connecting to AWS IOT Analytics.
    #[builder(into, default)]
    #[serde(rename = "awsIotAnalytics")]
    pub r#aws_iot_analytics: Box<Option<super::super::types::quicksight::DataSourceParametersAwsIotAnalytics>>,
    /// Parameters for connecting to Databricks.
    #[builder(into, default)]
    #[serde(rename = "databricks")]
    pub r#databricks: Box<Option<super::super::types::quicksight::DataSourceParametersDatabricks>>,
    /// Parameters for connecting to Jira.
    #[builder(into, default)]
    #[serde(rename = "jira")]
    pub r#jira: Box<Option<super::super::types::quicksight::DataSourceParametersJira>>,
    /// Parameters for connecting to MariaDB.
    #[builder(into, default)]
    #[serde(rename = "mariaDb")]
    pub r#maria_db: Box<Option<super::super::types::quicksight::DataSourceParametersMariaDb>>,
    /// Parameters for connecting to MySQL.
    #[builder(into, default)]
    #[serde(rename = "mysql")]
    pub r#mysql: Box<Option<super::super::types::quicksight::DataSourceParametersMysql>>,
    /// Parameters for connecting to Oracle.
    #[builder(into, default)]
    #[serde(rename = "oracle")]
    pub r#oracle: Box<Option<super::super::types::quicksight::DataSourceParametersOracle>>,
    /// Parameters for connecting to Postgresql.
    #[builder(into, default)]
    #[serde(rename = "postgresql")]
    pub r#postgresql: Box<Option<super::super::types::quicksight::DataSourceParametersPostgresql>>,
    /// Parameters for connecting to Presto.
    #[builder(into, default)]
    #[serde(rename = "presto")]
    pub r#presto: Box<Option<super::super::types::quicksight::DataSourceParametersPresto>>,
    /// Parameters for connecting to RDS.
    #[builder(into, default)]
    #[serde(rename = "rds")]
    pub r#rds: Box<Option<super::super::types::quicksight::DataSourceParametersRds>>,
    /// Parameters for connecting to Redshift.
    #[builder(into, default)]
    #[serde(rename = "redshift")]
    pub r#redshift: Box<Option<super::super::types::quicksight::DataSourceParametersRedshift>>,
    /// Parameters for connecting to S3.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::quicksight::DataSourceParametersS3>>,
    /// Parameters for connecting to ServiceNow.
    #[builder(into, default)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Box<Option<super::super::types::quicksight::DataSourceParametersServiceNow>>,
    /// Parameters for connecting to Snowflake.
    #[builder(into, default)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Box<Option<super::super::types::quicksight::DataSourceParametersSnowflake>>,
    /// Parameters for connecting to Spark.
    #[builder(into, default)]
    #[serde(rename = "spark")]
    pub r#spark: Box<Option<super::super::types::quicksight::DataSourceParametersSpark>>,
    /// Parameters for connecting to SQL Server.
    #[builder(into, default)]
    #[serde(rename = "sqlServer")]
    pub r#sql_server: Box<Option<super::super::types::quicksight::DataSourceParametersSqlServer>>,
    /// Parameters for connecting to Teradata.
    #[builder(into, default)]
    #[serde(rename = "teradata")]
    pub r#teradata: Box<Option<super::super::types::quicksight::DataSourceParametersTeradata>>,
    /// Parameters for connecting to Twitter.
    #[builder(into, default)]
    #[serde(rename = "twitter")]
    pub r#twitter: Box<Option<super::super::types::quicksight::DataSourceParametersTwitter>>,
}
