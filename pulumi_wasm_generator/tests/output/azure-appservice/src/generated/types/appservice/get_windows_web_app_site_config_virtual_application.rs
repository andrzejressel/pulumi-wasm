#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsWebAppSiteConfigVirtualApplication {
    /// The path on disk to the Virtual Directory
    #[builder(into)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: Box<String>,
    /// Is this Application Pre-loaded at startup.
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: Box<bool>,
    /// A `virtual_directory` block as defined below.
    #[builder(into)]
    #[serde(rename = "virtualDirectories")]
    pub r#virtual_directories: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigVirtualApplicationVirtualDirectory>>,
    /// The Virtual Path of the Virtual Directory.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: Box<String>,
}
