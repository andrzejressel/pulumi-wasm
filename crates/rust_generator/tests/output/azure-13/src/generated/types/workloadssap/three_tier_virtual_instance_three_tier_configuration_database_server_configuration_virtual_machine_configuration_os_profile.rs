#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfigurationVirtualMachineConfigurationOsProfile {
    /// The name of the administrator account. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// The SSH public key that is used to authenticate with the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sshPrivateKey")]
    pub r#ssh_private_key: Box<String>,
    /// The SSH private key that is used to authenticate with the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sshPublicKey")]
    pub r#ssh_public_key: Box<String>,
}
