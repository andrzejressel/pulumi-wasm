#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSiteConfigHandlerMapping {
    /// Specifies the command-line arguments to be passed to the script processor.
    #[builder(into, default)]
    #[serde(rename = "arguments")]
    pub r#arguments: Box<Option<String>>,
    /// Specifies which extension to be handled by the specified FastCGI application.
    #[builder(into)]
    #[serde(rename = "extension")]
    pub r#extension: Box<String>,
    /// Specifies the absolute path to the FastCGI application.
    #[builder(into)]
    #[serde(rename = "scriptProcessorPath")]
    pub r#script_processor_path: Box<String>,
}
