#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleAppVersionResources {
    /// Number of CPU cores needed.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<i32>>,
    /// Disk size (GB) needed.
    #[builder(into, default)]
    #[serde(rename = "diskGb")]
    pub r#disk_gb: Box<Option<i32>>,
    /// Memory (GB) needed.
    #[builder(into, default)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<Option<f64>>,
    /// List of ports, or port pairs, to forward from the virtual machine to the application container.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::appengine::FlexibleAppVersionResourcesVolume>>>,
}
