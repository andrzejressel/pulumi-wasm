#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsFunctionAppSlotSiteConfigApplicationStack {
    /// The version of .Net. Possible values are `v3.0`, `v4.0`, `v6.0`, `v7.0`, `v8.0` and `v9.0`. Defaults to `v4.0`.
    #[builder(into, default)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Box<Option<String>>,
    /// The version of Java to use. Possible values are `1.8`, `11` and `17` (In-Preview).
    #[builder(into, default)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<Option<String>>,
    /// The version of Node to use. Possible values are `~12`, `~14`, `~16`, `~18` and `~20`.
    #[builder(into, default)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Box<Option<String>>,
    /// The PowerShell Core version to use. Possible values are `7`, `7.2`, and `7.4`.
    #[builder(into, default)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: Box<Option<String>>,
    /// Does the Function App use a custom Application Stack?
    #[builder(into, default)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: Box<Option<bool>>,
    /// Should the DotNet process use an isolated runtime. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: Box<Option<bool>>,
}
