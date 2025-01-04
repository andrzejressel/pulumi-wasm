#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceRelationalDatabaseConfig {
    /// Amazon RDS HTTP endpoint configuration. See `http_endpoint_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "httpEndpointConfig")]
    pub r#http_endpoint_config: Box<Option<super::super::types::appsync::DataSourceRelationalDatabaseConfigHttpEndpointConfig>>,
    /// Source type for the relational database. Valid values: `RDS_HTTP_ENDPOINT`.
    #[builder(into, default)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<Option<String>>,
}
