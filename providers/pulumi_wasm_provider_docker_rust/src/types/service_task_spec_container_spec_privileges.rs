#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    /// CredentialSpec for managed service account (Windows only)
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    /// SELinux labels of the container
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}
