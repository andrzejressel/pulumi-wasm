#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfigVirtualApplication {
    /// The physical path for the Virtual Application.
    #[builder(into)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: Box<String>,
    /// Should pre-loading be enabled.
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: Box<bool>,
    /// One or more `virtual_directory` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "virtualDirectories")]
    pub r#virtual_directories: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigVirtualApplicationVirtualDirectory>>>,
    /// The Virtual Path for the Virtual Application.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: Box<String>,
}
