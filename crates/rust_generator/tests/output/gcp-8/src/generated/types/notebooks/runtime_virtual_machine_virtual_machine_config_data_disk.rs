#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuntimeVirtualMachineVirtualMachineConfigDataDisk {
    /// (Output)
    /// Optional. Specifies whether the disk will be auto-deleted
    /// when the instance is deleted (but not when the disk is
    /// detached from the instance).
    #[builder(into, default)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<Option<bool>>,
    /// (Output)
    /// Optional. Indicates that this is a boot disk. The virtual
    /// machine will use the first partition of the disk for its
    /// root filesystem.
    #[builder(into, default)]
    #[serde(rename = "boot")]
    pub r#boot: Box<Option<bool>>,
    /// (Output)
    /// Optional. Specifies a unique device name of your choice
    /// that is reflected into the /dev/disk/by-id/google-* tree
    /// of a Linux operating system running within the instance.
    /// This name can be used to reference the device for mounting,
    /// resizing, and so on, from within the instance.
    /// If not specified, the server chooses a default device name
    /// to apply to this disk, in the form persistent-disk-x, where
    /// x is a number assigned by Google Compute Engine. This field
    /// is only applicable for persistent disks.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// (Output)
    /// Indicates a list of features to enable on the guest operating
    /// system. Applicable only for bootable images. To see a list of
    /// available features, read `https://cloud.google.com/compute/docs/
    /// images/create-delete-deprecate-private-images#guest-os-features`
    /// options. ``
    #[builder(into, default)]
    #[serde(rename = "guestOsFeatures")]
    pub r#guest_os_features: Box<Option<Vec<String>>>,
    /// (Output)
    /// Output only. A zero-based index to this disk, where 0 is
    /// reserved for the boot disk. If you have many disks attached
    /// to an instance, each disk would have a unique index number.
    #[builder(into, default)]
    #[serde(rename = "index")]
    pub r#index: Box<Option<i32>>,
    /// Input only. Specifies the parameters for a new disk that will
    /// be created alongside the new instance. Use initialization
    /// parameters to create boot disks or local SSDs attached to the
    /// new instance. This property is mutually exclusive with the
    /// source property; you can only define one or the other, but not
    /// both.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Box<Option<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigDataDiskInitializeParams>>,
    /// "Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent
    /// disks must always use SCSI and the request will fail if you attempt
    /// to attach a persistent disk in any other format than SCSI. Local SSDs
    /// can use either NVME or SCSI. For performance characteristics of SCSI
    /// over NVMe, see Local SSD performance. Valid values: * NVME * SCSI".
    #[builder(into, default)]
    #[serde(rename = "interface")]
    pub r#interface: Box<Option<String>>,
    /// (Output)
    /// Type of the resource. Always compute#attachedDisk for attached
    /// disks.
    #[builder(into, default)]
    #[serde(rename = "kind")]
    pub r#kind: Box<Option<String>>,
    /// (Output)
    /// Output only. Any valid publicly visible licenses.
    #[builder(into, default)]
    #[serde(rename = "licenses")]
    pub r#licenses: Box<Option<Vec<String>>>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If not specified, the default is to attach
    /// the disk in READ_WRITE mode.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Specifies a valid partial or full URL to an existing
    /// Persistent Disk resource.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// Specifies the type of the disk, either SCRATCH or PERSISTENT.
    /// If not specified, the default is PERSISTENT.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
