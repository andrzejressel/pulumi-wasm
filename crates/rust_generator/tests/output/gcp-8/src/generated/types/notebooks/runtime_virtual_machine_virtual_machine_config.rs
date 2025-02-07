#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuntimeVirtualMachineVirtualMachineConfig {
    /// The Compute Engine accelerator configuration for this runtime.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "acceleratorConfig")]
    pub r#accelerator_config: Box<Option<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigAcceleratorConfig>>,
    /// Use a list of container images to start the notebook instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containerImages")]
    pub r#container_images: Box<Option<Vec<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigContainerImage>>>,
    /// Data disk option configuration settings.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataDisk")]
    pub r#data_disk: Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigDataDisk>,
    /// Encryption settings for virtual machine data disk.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<Option<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigEncryptionConfig>>,
    /// (Output)
    /// The Compute Engine guest attributes. (see [Project and instance
    /// guest attributes](https://cloud.google.com/compute/docs/
    /// storing-retrieving-metadata#guest_attributes)).
    #[builder(into, default)]
    #[serde(rename = "guestAttributes")]
    pub r#guest_attributes: Box<Option<std::collections::HashMap<String, String>>>,
    /// If true, runtime will only have internal IP addresses. By default,
    /// runtimes are not restricted to internal IP addresses, and will
    /// have ephemeral external IP addresses assigned to each vm. This
    /// `internal_ip_only` restriction can only be enabled for subnetwork
    /// enabled networks, and all dependencies must be configured to be
    /// accessible without external IP addresses.
    #[builder(into, default)]
    #[serde(rename = "internalIpOnly")]
    pub r#internal_ip_only: Box<Option<bool>>,
    /// The labels to associate with this runtime. Label **keys** must
    /// contain 1 to 63 characters, and must conform to [RFC 1035]
    /// (https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be
    /// empty, but, if present, must contain 1 to 63 characters, and must
    /// conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No
    /// more than 32 labels can be associated with a cluster.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Compute Engine machine type used for runtimes.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<String>,
    /// The Compute Engine metadata entries to add to virtual machine.
    /// (see [Project and instance metadata](https://cloud.google.com
    /// /compute/docs/storing-retrieving-metadata#project_and_instance
    /// _metadata)).
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Compute Engine network to be used for machine communications.
    /// Cannot be specified with subnetwork. If neither `network` nor
    /// `subnet` is specified, the "default" network of the project is
    /// used, if it exists. A full URL or partial URI. Examples:
    /// * `https://www.googleapis.com/compute/v1/projects/[project_id]/
    /// regions/global/default`
    /// * `projects/[project_id]/regions/global/default`
    /// Runtimes are managed resources inside Google Infrastructure.
    /// Runtimes support the following network configurations:
    /// * Google Managed Network (Network & subnet are empty)
    /// * Consumer Project VPC (network & subnet are required). Requires
    /// configuring Private Service Access.
    /// * Shared VPC (network & subnet are required). Requires
    /// configuring Private Service Access.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The type of vNIC to be used on this interface. This may be gVNIC
    /// or VirtioNet.
    /// Possible values are: `UNSPECIFIED_NIC_TYPE`, `VIRTIO_NET`, `GVNIC`.
    #[builder(into, default)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Box<Option<String>>,
    /// Reserved IP Range name is used for VPC Peering. The
    /// subnetwork allocation will use the range *name* if it's assigned.
    #[builder(into, default)]
    #[serde(rename = "reservedIpRange")]
    pub r#reserved_ip_range: Box<Option<String>>,
    /// Shielded VM Instance configuration settings.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigShieldedInstanceConfig>>,
    /// The Compute Engine subnetwork to be used for machine
    /// communications. Cannot be specified with network. A full URL or
    /// partial URI are valid. Examples:
    /// * `https://www.googleapis.com/compute/v1/projects/[project_id]/
    /// regions/us-east1/subnetworks/sub0`
    /// * `projects/[project_id]/regions/us-east1/subnetworks/sub0`
    #[builder(into, default)]
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
    /// The Compute Engine tags to add to runtime (see [Tagging instances]
    /// (https://cloud.google.com/compute/docs/
    /// label-or-tag-resources#tags)).
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// (Output)
    /// The zone where the virtual machine is located.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
