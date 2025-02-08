#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WindowsWebAppSlotSiteConfigVirtualApplicationVirtualDirectory {
    /// The physical path for the Virtual Application.
    #[builder(into, default)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: Box<Option<String>>,
    /// The Virtual Path for the Virtual Application.
    #[builder(into, default)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: Box<Option<String>>,
}
