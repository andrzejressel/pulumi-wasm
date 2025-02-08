#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuntimeVirtualMachineVirtualMachineConfigDataDiskInitializeParams {
    /// Provide this property when creating the disk.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies the disk name. If not specified, the default is
    /// to use the name of the instance. If the disk with the
    /// instance name exists already in the given zone/region, a
    /// new name will be automatically generated.
    #[builder(into, default)]
    #[serde(rename = "diskName")]
    pub r#disk_name: Box<Option<String>>,
    /// Specifies the size of the disk in base-2 GB. If not
    /// specified, the disk will be the same size as the image
    /// (usually 10GB). If specified, the size must be equal to
    /// or larger than 10GB. Default 100 GB.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// The type of the boot disk attached to this runtime,
    /// defaults to standard persistent disk. For valid values,
    /// see `https://cloud.google.com/vertex-ai/docs/workbench/
    /// reference/rest/v1/projects.locations.runtimes#disktype`
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// Labels to apply to this disk. These can be later modified
    /// by the disks.setLabels method. This field is only
    /// applicable for persistent disks.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
}
