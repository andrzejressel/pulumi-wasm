#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    /// CredentialSpec for managed service account (Windows only)
    #[builder(into, default)]
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec: Box<Option<super::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    /// SELinux labels of the container
    #[builder(into, default)]
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context: Box<Option<super::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}
