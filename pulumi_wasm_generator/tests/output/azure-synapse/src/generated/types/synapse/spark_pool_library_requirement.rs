#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SparkPoolLibraryRequirement {
    /// The content of library requirements.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// The name of the library requirements file.
    #[builder(into)]
    #[serde(rename = "filename")]
    pub r#filename: Box<String>,
}
