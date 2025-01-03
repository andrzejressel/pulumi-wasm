#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMachineTypesMachineTypeDeprecated {
    /// The URL of the suggested replacement for a deprecated machine type.
    #[builder(into)]
    #[serde(rename = "replacement")]
    pub r#replacement: Box<String>,
    /// The deprecation state of this resource. This can be `ACTIVE`, `DEPRECATED`, `OBSOLETE`, or `DELETED`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
