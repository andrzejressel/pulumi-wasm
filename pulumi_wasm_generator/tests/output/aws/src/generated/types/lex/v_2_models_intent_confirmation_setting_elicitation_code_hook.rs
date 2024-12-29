#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingElicitationCodeHook {
    /// Whether a Lambda function should be invoked for the dialog.
    #[builder(into, default)]
    #[serde(rename = "enableCodeHookInvocation")]
    pub r#enable_code_hook_invocation: Box<Option<bool>>,
    /// Label that indicates the dialog step from which the dialog code hook is happening.
    #[builder(into, default)]
    #[serde(rename = "invocationLabel")]
    pub r#invocation_label: Box<Option<String>>,
}
