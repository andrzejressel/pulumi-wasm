#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplateInstanceRequirementMemoryGibPerVcpus {
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Box<f64>,
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Box<f64>,
}
