#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolAccountRecoverySetting {
    #[builder(into)]
    #[serde(rename = "recoveryMechanisms")]
    pub r#recovery_mechanisms: Box<Vec<super::super::types::cognito::GetUserPoolAccountRecoverySettingRecoveryMechanism>>,
}