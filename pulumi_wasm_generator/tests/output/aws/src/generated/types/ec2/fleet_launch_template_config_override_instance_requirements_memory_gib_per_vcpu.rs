#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirementsMemoryGibPerVcpu {
    /// The maximum amount of memory per vCPU, in GiB. To specify no maximum limit, omit this parameter.
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<f64>>,
    /// The minimum amount of memory per vCPU, in GiB. To specify no minimum limit, omit this parameter.
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<f64>>,
}
