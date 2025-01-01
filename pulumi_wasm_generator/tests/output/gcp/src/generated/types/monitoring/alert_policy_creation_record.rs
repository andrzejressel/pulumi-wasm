#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyCreationRecord {
    /// (Output)
    /// When the change occurred.
    #[builder(into, default)]
    #[serde(rename = "mutateTime")]
    pub r#mutate_time: Box<Option<String>>,
    /// (Output)
    /// The email address of the user making the change.
    #[builder(into, default)]
    #[serde(rename = "mutatedBy")]
    pub r#mutated_by: Box<Option<String>>,
}
