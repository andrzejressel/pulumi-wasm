#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2VmDataDisk {
    /// The mode in which to attach this disk. If not specified, the default is READ_WRITE
    /// mode. Only applicable to dataDisks.
    /// Default value is `READ_WRITE`.
    /// Possible values are: `READ_WRITE`, `READ_ONLY`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Specifies the full path to an existing disk. For example:
    /// "projects/my-project/zones/us-central1-c/disks/my-disk".
    #[builder(into)]
    #[serde(rename = "sourceDisk")]
    pub r#source_disk: Box<String>,
}
