#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsWebAppSiteConfigVirtualApplicationVirtualDirectory {
    /// The path on disk to the Virtual Directory
    #[builder(into)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: Box<String>,
    /// The Virtual Path of the Virtual Directory.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: Box<String>,
}
