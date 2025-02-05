#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfigGuestAcceleratorGpuDriverInstallationConfig {
    /// Mode for how the GPU driver is installed.
    #[builder(into)]
    #[serde(rename = "gpuDriverVersion")]
    pub r#gpu_driver_version: Box<String>,
}
