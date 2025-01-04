#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineOsProfileLinuxConfigSshKey {
    /// The Public SSH Key which should be written to the `path` defined above.
    /// 
    /// > **Note:** Azure only supports RSA SSH2 key signatures of at least 2048 bits in length
    #[builder(into)]
    #[serde(rename = "keyData")]
    pub r#key_data: Box<String>,
    /// The path of the destination file on the virtual machine
    /// 
    /// > **NOTE:** Due to a limitation in the Azure VM Agent the only allowed `path` is `/home/{username}/.ssh/authorized_keys`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
