#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDefinitionHumanLoopConfigPublicWorkforceTaskPrice {
    /// Defines the amount of money paid to an Amazon Mechanical Turk worker in United States dollars. See Amount In Usd details below.
    #[builder(into, default)]
    #[serde(rename = "amountInUsd")]
    pub r#amount_in_usd: Box<Option<super::super::types::sagemaker::FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd>>,
}
