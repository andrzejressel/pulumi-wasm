#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IoTHubRoute {
    /// The condition that is evaluated to apply the routing rule. Defaults to `true`. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Used to specify whether a route is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The list of endpoints to which messages that satisfy the condition are routed.
    #[builder(into)]
    #[serde(rename = "endpointNames")]
    pub r#endpoint_names: Box<Vec<String>>,
    /// The name of the route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The source that the routing rule is to be applied to, such as `DeviceMessages`. Possible values include: `Invalid`, `DeviceMessages`, `TwinChangeEvents`, `DeviceLifecycleEvents`, `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents` and `DigitalTwinChangeEvents`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
