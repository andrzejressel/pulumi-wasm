#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationWinrmListener {
    /// The Secret URL of a Key Vault Certificate, which must be specified when protocol is set to `Https`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This can be sourced from the `secret_id` field within the `azure.keyvault.Certificate` Resource.
    #[builder(into, default)]
    #[serde(rename = "certificateUrl")]
    pub r#certificate_url: Box<Option<String>>,
    /// Specifies the protocol of listener. Possible values are `Http` or `Https`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
}