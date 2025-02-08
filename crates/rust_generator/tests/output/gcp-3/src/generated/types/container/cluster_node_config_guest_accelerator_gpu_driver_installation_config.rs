#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeConfigGuestAcceleratorGpuDriverInstallationConfig {
    /// Mode for how the GPU driver is installed.
    /// Accepted values are:
    /// * `"GPU_DRIVER_VERSION_UNSPECIFIED"`: Default value is to not install any GPU driver.
    /// * `"INSTALLATION_DISABLED"`: Disable GPU driver auto installation and needs manual installation.
    /// * `"DEFAULT"`: "Default" GPU driver in COS and Ubuntu.
    /// * `"LATEST"`: "Latest" GPU driver in COS.
    #[builder(into)]
    #[serde(rename = "gpuDriverVersion")]
    pub r#gpu_driver_version: Box<String>,
}
