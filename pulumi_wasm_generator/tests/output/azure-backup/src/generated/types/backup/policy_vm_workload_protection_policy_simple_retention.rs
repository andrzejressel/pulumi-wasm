#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyVmWorkloadProtectionPolicySimpleRetention {
    /// The count that is used to count retention duration with duration type `Days`. Possible values are between `7` and `35`.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
}
