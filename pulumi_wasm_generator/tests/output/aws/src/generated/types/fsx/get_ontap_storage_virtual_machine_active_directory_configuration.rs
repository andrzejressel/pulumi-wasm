#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOntapStorageVirtualMachineActiveDirectoryConfiguration {
    /// The NetBIOS name of the AD computer object to which the SVM is joined.
    #[builder(into)]
    #[serde(rename = "netbiosName")]
    pub r#netbios_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "selfManagedActiveDirectoryConfigurations")]
    pub r#self_managed_active_directory_configurations: Box<Vec<super::super::types::fsx::GetOntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration>>,
}