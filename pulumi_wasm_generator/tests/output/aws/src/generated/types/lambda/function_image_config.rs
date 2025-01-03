#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionImageConfig {
    /// Parameters that you want to pass in with `entry_point`.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// Entry point to your application, which is typically the location of the runtime executable.
    #[builder(into, default)]
    #[serde(rename = "entryPoints")]
    pub r#entry_points: Box<Option<Vec<String>>>,
    /// Working directory.
    #[builder(into, default)]
    #[serde(rename = "workingDirectory")]
    pub r#working_directory: Box<Option<String>>,
}
