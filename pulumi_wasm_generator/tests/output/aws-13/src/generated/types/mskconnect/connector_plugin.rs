#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorPlugin {
    /// Details about a custom plugin. See `custom_plugin` Block for details.
    #[builder(into)]
    #[serde(rename = "customPlugin")]
    pub r#custom_plugin: Box<super::super::types::mskconnect::ConnectorPluginCustomPlugin>,
}
