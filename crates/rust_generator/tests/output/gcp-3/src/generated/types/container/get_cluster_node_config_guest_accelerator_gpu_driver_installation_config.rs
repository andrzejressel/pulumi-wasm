#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterNodeConfigGuestAcceleratorGpuDriverInstallationConfig {
    /// Mode for how the GPU driver is installed.
    #[builder(into)]
    #[serde(rename = "gpuDriverVersion")]
    pub r#gpu_driver_version: Box<String>,
}
