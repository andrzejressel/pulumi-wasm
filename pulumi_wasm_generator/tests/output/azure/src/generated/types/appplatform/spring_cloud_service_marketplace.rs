#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceMarketplace {
    /// Specifies the plan ID of the 3rd Party Artifact that is being procured.
    #[builder(into)]
    #[serde(rename = "plan")]
    pub r#plan: Box<String>,
    /// Specifies the 3rd Party artifact that is being procured.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// Specifies the publisher ID of the 3rd Party Artifact that is being procured.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
}