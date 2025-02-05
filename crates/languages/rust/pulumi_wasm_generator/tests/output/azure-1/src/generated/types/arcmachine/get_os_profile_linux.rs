#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOsProfileLinux {
    /// A `patch` block as defined above.
    #[builder(into)]
    #[serde(rename = "patches")]
    pub r#patches: Box<Vec<super::super::types::arcmachine::GetOsProfileLinuxPatch>>,
}
