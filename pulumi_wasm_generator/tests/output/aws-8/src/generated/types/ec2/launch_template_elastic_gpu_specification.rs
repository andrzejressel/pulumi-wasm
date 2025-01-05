#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateElasticGpuSpecification {
    /// The [Elastic GPU Type](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-graphics.html#elastic-graphics-basics)
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
