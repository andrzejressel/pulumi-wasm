#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDiskGuestOsFeature {
    /// URL of the disk type resource describing which disk type to use to
    /// create the disk.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
