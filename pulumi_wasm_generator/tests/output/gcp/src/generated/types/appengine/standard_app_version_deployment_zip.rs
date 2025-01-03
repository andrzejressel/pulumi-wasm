#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardAppVersionDeploymentZip {
    /// files count
    #[builder(into, default)]
    #[serde(rename = "filesCount")]
    pub r#files_count: Box<Option<i32>>,
    /// Source URL
    #[builder(into)]
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
}
