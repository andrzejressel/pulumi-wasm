#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMachineTypesMachineType {
    /// A list of accelerator configurations assigned to this machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Vec<super::super::types::compute::GetMachineTypesMachineTypeAccelerator>>,
    /// The configuration of bundled local SSD for the machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bundledLocalSsds")]
    pub r#bundled_local_ssds: Box<Vec<super::super::types::compute::GetMachineTypesMachineTypeBundledLocalSsd>>,
    /// The deprecation status associated with this machine type. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "deprecateds")]
    pub r#deprecateds: Box<Vec<super::super::types::compute::GetMachineTypesMachineTypeDeprecated>>,
    /// A textual description of the machine type.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The number of virtual CPUs that are available to the instance.
    #[builder(into)]
    #[serde(rename = "guestCpus")]
    pub r#guest_cpus: Box<i32>,
    /// Whether this machine type has a shared CPU.
    #[builder(into)]
    #[serde(rename = "isSharedCpus")]
    pub r#is_shared_cpus: Box<bool>,
    /// The maximum persistent disks allowed.
    #[builder(into)]
    #[serde(rename = "maximumPersistentDisks")]
    pub r#maximum_persistent_disks: Box<i32>,
    /// The maximum total persistent disks size (GB) allowed.
    #[builder(into)]
    #[serde(rename = "maximumPersistentDisksSizeGb")]
    pub r#maximum_persistent_disks_size_gb: Box<i32>,
    /// The amount of physical memory available to the instance, defined in MB.
    #[builder(into)]
    #[serde(rename = "memoryMb")]
    pub r#memory_mb: Box<i32>,
    /// The name of the machine type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The server-defined URL for the machine type.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
}
