#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppImageConfigKernelGatewayImageConfig {
    /// The URL where the Git repository is located. See File System Config details below.
    #[builder(into, default)]
    #[serde(rename = "fileSystemConfig")]
    pub r#file_system_config: Box<Option<super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfigFileSystemConfig>>,
    /// The default branch for the Git repository. See Kernel Spec details below.
    #[builder(into)]
    #[serde(rename = "kernelSpec")]
    pub r#kernel_spec: Box<super::super::types::sagemaker::AppImageConfigKernelGatewayImageConfigKernelSpec>,
}
