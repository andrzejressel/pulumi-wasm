#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HadoopClusterNetwork {
    /// The direction of the resource provider connection. Possible values include `Inbound` or `Outbound`. Defaults to `Inbound`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** To enabled the private link the `connection_direction` must be set to `Outbound`.
    #[builder(into, default)]
    #[serde(rename = "connectionDirection")]
    pub r#connection_direction: Box<Option<String>>,
    /// Is the private link enabled? Possible values include `true` or `false`. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "privateLinkEnabled")]
    pub r#private_link_enabled: Box<Option<bool>>,
}
