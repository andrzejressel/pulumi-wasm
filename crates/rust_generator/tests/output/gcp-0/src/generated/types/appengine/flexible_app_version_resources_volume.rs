#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleAppVersionResourcesVolume {
    /// Unique name for the volume.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Volume size in gigabytes.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<i32>,
    /// Underlying volume type, e.g. 'tmpfs'.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<String>,
}
