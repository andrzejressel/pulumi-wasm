#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPlanRuleCopyAction {
    #[builder(into)]
    #[serde(rename = "destinationVaultArn")]
    pub r#destination_vault_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "lifecycles")]
    pub r#lifecycles: Box<Vec<super::super::types::backup::GetPlanRuleCopyActionLifecycle>>,
}