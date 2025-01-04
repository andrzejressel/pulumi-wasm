#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IotHubDpsSku {
    /// The number of provisioned IoT Device Provisioning Service units.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// The name of the sku. Currently can only be set to `S1`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
