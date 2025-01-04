#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationAction {
    /// (Optional, but required when `type` is `eventhub`) A connection string to send data to the target Event Hub namespace, this should include a key with send permissions.
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The resource id of the target Logic App, Event Hub namespace or Log Analytics workspace.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    /// (Optional, but required when `type` is `logicapp`) The callback URL to trigger the Logic App that will receive and process data sent by this automation. This can be found in the Azure Portal under "See trigger history"
    #[builder(into, default)]
    #[serde(rename = "triggerUrl")]
    pub r#trigger_url: Box<Option<String>>,
    /// Type of Azure resource to send data to. Must be set to one of: `logicapp`, `eventhub` or `loganalytics`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
