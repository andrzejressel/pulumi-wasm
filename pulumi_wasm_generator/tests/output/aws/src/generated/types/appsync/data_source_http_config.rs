#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceHttpConfig {
    /// Authorization configuration in case the HTTP endpoint requires authorization. See `authorization_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "authorizationConfig")]
    pub r#authorization_config: Box<Option<super::super::types::appsync::DataSourceHttpConfigAuthorizationConfig>>,
    /// HTTP URL.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
}