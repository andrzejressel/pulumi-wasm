#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxFunctionAppSiteConfigApplicationStack {
    /// One or more `docker` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dockers")]
    pub r#dockers: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigApplicationStackDocker>>,
    /// The version of .NET used.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Box<String>,
    /// The Version of Java used.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Box<String>,
    /// The version of Node used.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Box<String>,
    /// The version of PowerShell Core used.
    #[builder(into)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: Box<String>,
    /// The version of Python used.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Box<String>,
    /// Does the Linux Function App use a custom runtime?
    #[builder(into)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: Box<bool>,
    #[builder(into)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: Box<bool>,
}
