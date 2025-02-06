#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineOsProfileWindowsConfigWinrm {
    /// The ID of the Key Vault Secret which contains the encrypted Certificate which should be installed on the Virtual Machine. This certificate must also be specified in the `vault_certificates` block within the `os_profile_secrets` block.
    /// 
    /// > **NOTE:** This can be sourced from the `secret_id` field on the `azure.keyvault.Certificate` resource.
    #[builder(into, default)]
    #[serde(rename = "certificateUrl")]
    pub r#certificate_url: Box<Option<String>>,
    /// Specifies the protocol of listener. Possible values are `HTTP` or `HTTPS`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}
