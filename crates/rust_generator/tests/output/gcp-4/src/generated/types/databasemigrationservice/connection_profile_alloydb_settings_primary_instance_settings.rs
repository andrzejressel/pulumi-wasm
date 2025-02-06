#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileAlloydbSettingsPrimaryInstanceSettings {
    /// Database flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used.
    #[builder(into, default)]
    #[serde(rename = "databaseFlags")]
    pub r#database_flags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The database username.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Labels for the AlloyDB primary instance created by DMS.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Configuration for the machines that host the underlying database engine.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "machineConfig")]
    pub r#machine_config: Box<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettingsPrimaryInstanceSettingsMachineConfig>,
    /// (Output)
    /// Output only. The private IP address for the Instance. This is the connection endpoint for an end-user application.
    #[builder(into, default)]
    #[serde(rename = "privateIp")]
    pub r#private_ip: Box<Option<String>>,
}
