#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamSnowflakeConfigurationSnowflakeRoleConfiguration {
    /// Whether the Snowflake role is enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The Snowflake role.
    #[builder(into, default)]
    #[serde(rename = "snowflakeRole")]
    pub r#snowflake_role: Box<Option<String>>,
}