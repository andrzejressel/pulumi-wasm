#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReleaseLabelsFilters {
    /// Optional release label application filter. For example, `Spark@2.1.0` or `Spark`.
    #[builder(into, default)]
    #[serde(rename = "application")]
    pub r#application: Box<Option<String>>,
    /// Optional release label version prefix filter. For example, `emr-5`.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
