#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesPostgresqlFlexibleServer {
    #[builder(into, default)]
    #[serde(rename = "restartServerOnConfigurationValueChange")]
    pub r#restart_server_on_configuration_value_change: Box<Option<bool>>,
}
