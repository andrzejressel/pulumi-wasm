#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpringCloudAppPersistentDisk {
    /// Specifies the mount path of the persistent disk. Defaults to `/persistent`.
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
    /// Specifies the size of the persistent disk in GB. Possible values are between `0` and `50`.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: Box<i32>,
}
