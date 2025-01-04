#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineAdminSshKey {
    /// The Public Key which should be used for authentication, which needs to be in `ssh-rsa` format with at least 2048-bit or in `ssh-ed25519` format. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
    /// The Username for which this Public SSH Key should be configured. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** The Azure VM Agent only allows creating SSH Keys at the path `/home/{username}/.ssh/authorized_keys` - as such this public key will be written to the authorized keys file.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
