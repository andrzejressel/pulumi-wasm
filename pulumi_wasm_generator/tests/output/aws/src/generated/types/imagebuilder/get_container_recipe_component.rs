#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContainerRecipeComponent {
    /// ARN of the Image Builder Component.
    #[builder(into)]
    #[serde(rename = "componentArn")]
    pub r#component_arn: Box<String>,
    /// Set of parameters that are used to configure the component.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Vec<super::super::types::imagebuilder::GetContainerRecipeComponentParameter>>,
}
