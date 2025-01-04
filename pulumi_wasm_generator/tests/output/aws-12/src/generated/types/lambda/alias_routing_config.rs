#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AliasRoutingConfig {
    /// A map that defines the proportion of events that should be sent to different versions of a lambda function.
    #[builder(into, default)]
    #[serde(rename = "additionalVersionWeights")]
    pub r#additional_version_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}
