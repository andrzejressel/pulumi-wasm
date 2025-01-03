#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntentFulfillmentActivity {
    /// A description of the Lambda function that is run to fulfill the intent.
    /// Required if type is CodeHook. Attributes are documented under code_hook.
    #[builder(into, default)]
    #[serde(rename = "codeHook")]
    pub r#code_hook: Box<Option<super::super::types::lex::IntentFulfillmentActivityCodeHook>>,
    /// How the intent should be fulfilled, either by running a Lambda function or by
    /// returning the slot data to the client application. Type can be either `ReturnIntent` or `CodeHook`, as documented [here](https://docs.aws.amazon.com/lex/latest/dg/API_FulfillmentActivity.html).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
