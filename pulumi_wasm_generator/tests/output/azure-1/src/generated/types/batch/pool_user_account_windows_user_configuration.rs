#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolUserAccountWindowsUserConfiguration {
    /// Specifies login mode for the user. The default value for VirtualMachineConfiguration pools is interactive mode and for CloudServiceConfiguration pools is batch mode. Values supported are "Batch" and "Interactive".
    #[builder(into)]
    #[serde(rename = "loginMode")]
    pub r#login_mode: Box<String>,
}
