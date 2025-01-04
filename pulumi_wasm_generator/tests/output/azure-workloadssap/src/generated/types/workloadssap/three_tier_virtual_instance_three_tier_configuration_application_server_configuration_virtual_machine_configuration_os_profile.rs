#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationApplicationServerConfigurationVirtualMachineConfigurationOsProfile {
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
