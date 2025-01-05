#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateCreditSpecification {
    /// The credit option for CPU usage.
    /// Can be `standard` or `unlimited`.
    /// T3 instances are launched as `unlimited` by default.
    /// T2 instances are launched as `standard` by default.
    #[builder(into, default)]
    #[serde(rename = "cpuCredits")]
    pub r#cpu_credits: Box<Option<String>>,
}
