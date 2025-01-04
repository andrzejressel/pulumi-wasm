#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateVolumeGcs {
    /// GCS Bucket name
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// A list of flags to pass to the gcsfuse command for configuring this volume.
    /// Flags should be passed without leading dashes.
    #[builder(into, default)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<Option<Vec<String>>>,
    /// If true, mount the GCS bucket as read-only
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
}
