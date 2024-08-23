#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}
