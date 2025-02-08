#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceParametersSnowflake {
    /// The database to which to connect.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// The host to which to connect.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The warehouse to which to connect.
    #[builder(into)]
    #[serde(rename = "warehouse")]
    pub r#warehouse: Box<String>,
}
