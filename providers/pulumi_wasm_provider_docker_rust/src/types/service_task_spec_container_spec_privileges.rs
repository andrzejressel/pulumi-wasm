#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    /// CredentialSpec for managed service account (Windows only)
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    /// SELinux labels of the container
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}
