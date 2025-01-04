#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFunctionAppSiteConfigApplicationStack {
    /// The version of .Net to use.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Box<String>,
    /// The version of Java to use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<String>,
    /// The version of Node to use.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Box<String>,
    /// The version of PowerShell Core to use.
    #[builder(into)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: Box<String>,
    /// Is the Windows Function App using a custom runtime?.
    #[builder(into)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: Box<bool>,
    #[builder(into)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: Box<bool>,
}
