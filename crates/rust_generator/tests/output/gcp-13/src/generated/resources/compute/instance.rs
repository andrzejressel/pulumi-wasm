/// Manages a VM instance resource within GCE. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/instances)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/instances).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-custom-sa
///       displayName: Custom SA for VM Instance
///   defaultInstance:
///     type: gcp:compute:Instance
///     name: default
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: default
///       name: my-instance
///       machineType: n2-standard-2
///       zone: us-central1-a
///       tags:
///         - foo
///         - bar
///       bootDisk:
///         initializeParams:
///           image: debian-cloud/debian-11
///           labels:
///             my_label: value
///       scratchDisks:
///         - interface: NVME
///       metadata:
///         foo: bar
///       metadataStartupScript: echo hi > /test.txt
///       serviceAccount:
///         email: ${default.email}
///         scopes:
///           - cloud-platform
/// ```
///
///
/// ### Confidential Computing
///
/// Example with [Confidential Mode](https://cloud.google.com/confidential-computing/confidential-vm/docs/confidential-vm-overview) activated.
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-custom-sa
///       displayName: Custom SA for VM Instance
///   confidentialInstance:
///     type: gcp:compute:Instance
///     name: confidential_instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: default
///       name: my-confidential-instance
///       zone: us-central1-a
///       machineType: n2d-standard-2
///       minCpuPlatform: AMD Milan
///       confidentialInstanceConfig:
///         enableConfidentialCompute: true
///         confidentialInstanceType: SEV
///       bootDisk:
///         initializeParams:
///           image: ubuntu-os-cloud/ubuntu-2004-lts
///           labels:
///             my_label: value
///       scratchDisks:
///         - interface: NVME
///       serviceAccount:
///         email: ${default.email}
///         scopes:
///           - cloud-platform
/// ```
///
/// ## Import
///
/// Instances can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/instances/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, instances can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/instance:Instance default projects/{{project}}/zones/{{zone}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instance:Instance default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/instance:Instance default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Configure Nested Virtualisation and Simultaneous Hyper Threading  on this VM. Structure is documented below
        #[builder(into, default)]
        pub advanced_machine_features: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceAdvancedMachineFeatures>,
        >,
        /// If true, allows this prvider to stop the instance to update its properties.
        /// If you try to update a property that requires stopping the instance without setting this field, the update will fail.
        #[builder(into, default)]
        pub allow_stopping_for_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Additional disks to attach to the instance. Can be repeated multiple times for multiple disks. Structure is documented below.
        #[builder(into, default)]
        pub attached_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceAttachedDisk>>,
        >,
        /// The boot disk for the instance.
        /// Structure is documented below.
        #[builder(into)]
        pub boot_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::InstanceBootDisk,
        >,
        /// Whether to allow sending and receiving of
        /// packets with non-matching source or destination IPs.
        /// This defaults to false.
        #[builder(into, default)]
        pub can_ip_forward: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable [Confidential Mode](https://cloud.google.com/compute/confidential-vm/docs/about-cvm) on this VM. Structure is documented below
        #[builder(into, default)]
        pub confidential_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceConfidentialInstanceConfig>,
        >,
        /// Enable deletion protection on this instance. Defaults to false.
        /// **Note:** you must disable deletion protection before removing the resource (e.g., via `pulumi destroy`), or the instance cannot be deleted and the provider run will not complete successfully.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A brief description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Desired status of the instance. Either
        /// `"RUNNING"`, `"SUSPENDED"` or `"TERMINATED"`.
        #[builder(into, default)]
        pub desired_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable [Virtual Displays](https://cloud.google.com/compute/docs/instances/enable-instance-virtual-display#verify_display_driver) on this instance.
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        #[builder(into, default)]
        pub enable_display: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance. Structure documented below.
        /// **Note:** GPU accelerators can only be used with `on_host_maintenance` option set to TERMINATE.
        #[builder(into, default)]
        pub guest_accelerators: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceGuestAccelerator>>,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid.
        /// Valid format is a series of labels 1-63 characters long matching the regular expression `a-z`, concatenated with periods.
        /// The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Action to be taken when a customer's encryption key is revoked. Supports `STOP` and `NONE`, with `NONE` being the default.
        #[builder(into, default)]
        pub key_revocation_action_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of key/value label pairs to assign to the instance.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        ///
        /// **Note:** If you want to update this value (resize the VM) after initial creation, you must set `allow_stopping_for_update` to `true`.
        ///
        /// [Custom machine types](https://cloud.google.com/dataproc/docs/concepts/compute/custom-machine-types) can be formatted as `custom-NUMBER_OF_CPUS-AMOUNT_OF_MEMORY_MB`, e.g. `custom-6-20480` for 6 vCPU and 20GB of RAM.
        /// Because of current API limitations some custom machine types may get converted to different machine types (such as an equivalent standard type) and cause non-empty plans in your configuration. Use
        /// `lifecycle.ignore_changes` on `machine_type` in these cases.
        ///
        /// There is a limit of 6.5 GB per CPU unless you add [extended memory](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#extendedmemory). You must do this explicitly by adding the suffix `-ext`, e.g. `custom-2-15360-ext` for 2 vCPU and 15 GB of memory.
        #[builder(into)]
        pub machine_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata key/value pairs to make available from
        /// within the instance. Ssh keys attached in the Cloud Console will be removed.
        /// Add them to your config in order to keep them attached to your instance.
        /// A list of predefined metadata keys (e.g. ssh-keys) can be found [here](https://cloud.google.com/compute/docs/metadata/predefined-metadata-keys)
        ///
        /// > Depending on the OS you choose for your instance, some metadata keys have
        /// special functionality.  Most linux-based images will run the content of
        /// `metadata.startup-script` in a shell on every boot.  At a minimum,
        /// Debian, CentOS, RHEL, SLES, Container-Optimized OS, and Ubuntu images
        /// support this key.  Windows instances require other keys depending on the format
        /// of the script and the time you would like it to run - see [this table](https://cloud.google.com/compute/docs/startupscript#providing_a_startup_script_for_windows_instances).
        /// For the convenience of the users of `metadata.startup-script`,
        /// we provide a special attribute, `metadata_startup_script`, which is documented below.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An alternative to using the
        /// startup-script metadata key, except this one forces the instance to be recreated
        /// (thus re-running the script) if it is changed. This replaces the startup-script
        /// metadata key on the created instance and thus the two mechanisms are not
        /// allowed to be used simultaneously.  Users are free to use either mechanism - the
        /// only distinction is that this separate attribute will cause a recreate on
        /// modification.  On import, `metadata_startup_script` will not be set - if you
        /// choose to specify it you will see a diff immediately after import causing a
        /// destroy/recreate operation. If importing an instance and specifying this value
        /// is desired, you will need to modify your state file.
        #[builder(into, default)]
        pub metadata_startup_script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a minimum CPU platform for the VM instance. Applicable values are the friendly names of CPU platforms, such as
        /// `Intel Haswell` or `Intel Skylake`. See the complete list [here](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform).
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        #[builder(into, default)]
        pub min_cpu_platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Networks to attach to the instance. This can
        /// be specified multiple times. Structure is documented below.
        ///
        /// - - -
        #[builder(into)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::compute::InstanceNetworkInterface>,
        >,
        /// (Optional, Beta
        /// Configures network performance settings for the instance. Structure is
        /// documented below. **Note**: `machine_type` must be a [supported type](https://cloud.google.com/compute/docs/networking/configure-vm-with-high-bandwidth-configuration),
        /// the `image` used must include the [`GVNIC`](https://cloud.google.com/compute/docs/networking/using-gvnic#create-instance-gvnic-image)
        /// in `guest-os-features`, and `network_interface.0.nic-type` must be `GVNIC`
        /// in order for this setting to take effect.
        #[builder(into, default)]
        pub network_performance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceNetworkPerformanceConfig>,
        >,
        /// Additional instance parameters.
        /// .
        #[builder(into, default)]
        pub params: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceParams>,
        >,
        /// Beta key/value pair represents partner metadata assigned to instance where key represent a defined namespace and value is a json string represent the entries associted with the namespace.
        #[builder(into, default)]
        pub partner_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the reservations that this instance can consume from.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reservation_affinity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceReservationAffinity>,
        >,
        /// - A list of self_links of resource policies to attach to the instance. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
        #[builder(into, default)]
        pub resource_policies: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scheduling strategy to use. More details about
        /// this configuration option are detailed below.
        #[builder(into, default)]
        pub scheduling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceScheduling>,
        >,
        /// Scratch disks to attach to the instance. This can be
        /// specified multiple times for multiple scratch disks. Structure is documented below.
        #[builder(into, default)]
        pub scratch_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceScratchDisk>>,
        >,
        /// Service account to attach to the instance.
        /// Structure is documented below.
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceServiceAccount>,
        >,
        /// Enable [Shielded VM](https://cloud.google.com/security/shielded-cloud/shielded-vm) on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Structure is documented below.
        /// **Note**: `shielded_instance_config` can only be used with boot images with shielded vm support. See the complete list [here](https://cloud.google.com/compute/docs/images#shielded-images).
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceShieldedInstanceConfig>,
        >,
        /// A list of network tags to attach to the instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The zone that the machine should be created in. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Configure Nested Virtualisation and Simultaneous Hyper Threading  on this VM. Structure is documented below
        pub advanced_machine_features: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceAdvancedMachineFeatures>,
        >,
        /// If true, allows this prvider to stop the instance to update its properties.
        /// If you try to update a property that requires stopping the instance without setting this field, the update will fail.
        pub allow_stopping_for_update: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Additional disks to attach to the instance. Can be repeated multiple times for multiple disks. Structure is documented below.
        pub attached_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceAttachedDisk>>,
        >,
        /// The boot disk for the instance.
        /// Structure is documented below.
        pub boot_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceBootDisk,
        >,
        /// Whether to allow sending and receiving of
        /// packets with non-matching source or destination IPs.
        /// This defaults to false.
        pub can_ip_forward: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Enable [Confidential Mode](https://cloud.google.com/compute/confidential-vm/docs/about-cvm) on this VM. Structure is documented below
        pub confidential_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceConfidentialInstanceConfig,
        >,
        /// The CPU platform used by this instance.
        pub cpu_platform: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The current status of the instance. This could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle).
        pub current_status: pulumi_gestalt_rust::Output<String>,
        /// Enable deletion protection on this instance. Defaults to false.
        /// **Note:** you must disable deletion protection before removing the resource (e.g., via `pulumi destroy`), or the instance cannot be deleted and the provider run will not complete successfully.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A brief description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Desired status of the instance. Either
        /// `"RUNNING"`, `"SUSPENDED"` or `"TERMINATED"`.
        pub desired_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable [Virtual Displays](https://cloud.google.com/compute/docs/instances/enable-instance-virtual-display#verify_display_driver) on this instance.
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        pub enable_display: pulumi_gestalt_rust::Output<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance. Structure documented below.
        /// **Note:** GPU accelerators can only be used with `on_host_maintenance` option set to TERMINATE.
        pub guest_accelerators: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceGuestAccelerator>,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid.
        /// Valid format is a series of labels 1-63 characters long matching the regular expression `a-z`, concatenated with periods.
        /// The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created.
        pub hostname: pulumi_gestalt_rust::Output<Option<String>>,
        /// The server-assigned unique identifier of this instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked. Supports `STOP` and `NONE`, with `NONE` being the default.
        pub key_revocation_action_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique fingerprint of the labels.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// A map of key/value label pairs to assign to the instance.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        ///
        /// **Note:** If you want to update this value (resize the VM) after initial creation, you must set `allow_stopping_for_update` to `true`.
        ///
        /// [Custom machine types](https://cloud.google.com/dataproc/docs/concepts/compute/custom-machine-types) can be formatted as `custom-NUMBER_OF_CPUS-AMOUNT_OF_MEMORY_MB`, e.g. `custom-6-20480` for 6 vCPU and 20GB of RAM.
        /// Because of current API limitations some custom machine types may get converted to different machine types (such as an equivalent standard type) and cause non-empty plans in your configuration. Use
        /// `lifecycle.ignore_changes` on `machine_type` in these cases.
        ///
        /// There is a limit of 6.5 GB per CPU unless you add [extended memory](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#extendedmemory). You must do this explicitly by adding the suffix `-ext`, e.g. `custom-2-15360-ext` for 2 vCPU and 15 GB of memory.
        pub machine_type: pulumi_gestalt_rust::Output<String>,
        /// Metadata key/value pairs to make available from
        /// within the instance. Ssh keys attached in the Cloud Console will be removed.
        /// Add them to your config in order to keep them attached to your instance.
        /// A list of predefined metadata keys (e.g. ssh-keys) can be found [here](https://cloud.google.com/compute/docs/metadata/predefined-metadata-keys)
        ///
        /// > Depending on the OS you choose for your instance, some metadata keys have
        /// special functionality.  Most linux-based images will run the content of
        /// `metadata.startup-script` in a shell on every boot.  At a minimum,
        /// Debian, CentOS, RHEL, SLES, Container-Optimized OS, and Ubuntu images
        /// support this key.  Windows instances require other keys depending on the format
        /// of the script and the time you would like it to run - see [this table](https://cloud.google.com/compute/docs/startupscript#providing_a_startup_script_for_windows_instances).
        /// For the convenience of the users of `metadata.startup-script`,
        /// we provide a special attribute, `metadata_startup_script`, which is documented below.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// An alternative to using the
        /// startup-script metadata key, except this one forces the instance to be recreated
        /// (thus re-running the script) if it is changed. This replaces the startup-script
        /// metadata key on the created instance and thus the two mechanisms are not
        /// allowed to be used simultaneously.  Users are free to use either mechanism - the
        /// only distinction is that this separate attribute will cause a recreate on
        /// modification.  On import, `metadata_startup_script` will not be set - if you
        /// choose to specify it you will see a diff immediately after import causing a
        /// destroy/recreate operation. If importing an instance and specifying this value
        /// is desired, you will need to modify your state file.
        pub metadata_startup_script: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a minimum CPU platform for the VM instance. Applicable values are the friendly names of CPU platforms, such as
        /// `Intel Haswell` or `Intel Skylake`. See the complete list [here](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform).
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        pub min_cpu_platform: pulumi_gestalt_rust::Output<String>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Networks to attach to the instance. This can
        /// be specified multiple times. Structure is documented below.
        ///
        /// - - -
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceNetworkInterface>,
        >,
        /// (Optional, Beta
        /// Configures network performance settings for the instance. Structure is
        /// documented below. **Note**: `machine_type` must be a [supported type](https://cloud.google.com/compute/docs/networking/configure-vm-with-high-bandwidth-configuration),
        /// the `image` used must include the [`GVNIC`](https://cloud.google.com/compute/docs/networking/using-gvnic#create-instance-gvnic-image)
        /// in `guest-os-features`, and `network_interface.0.nic-type` must be `GVNIC`
        /// in order for this setting to take effect.
        pub network_performance_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceNetworkPerformanceConfig>,
        >,
        /// Additional instance parameters.
        /// .
        pub params: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceParams>,
        >,
        /// Beta key/value pair represents partner metadata assigned to instance where key represent a defined namespace and value is a json string represent the entries associted with the namespace.
        pub partner_metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the reservations that this instance can consume from.
        /// Structure is documented below.
        pub reservation_affinity: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceReservationAffinity,
        >,
        /// - A list of self_links of resource policies to attach to the instance. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_gestalt_rust::Output<Option<String>>,
        /// The scheduling strategy to use. More details about
        /// this configuration option are detailed below.
        pub scheduling: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceScheduling,
        >,
        /// Scratch disks to attach to the instance. This can be
        /// specified multiple times for multiple scratch disks. Structure is documented below.
        pub scratch_disks: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::InstanceScratchDisk>>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Service account to attach to the instance.
        /// Structure is documented below.
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        pub service_account: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::InstanceServiceAccount>,
        >,
        /// Enable [Shielded VM](https://cloud.google.com/security/shielded-cloud/shielded-vm) on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Structure is documented below.
        /// **Note**: `shielded_instance_config` can only be used with boot images with shielded vm support. See the complete list [here](https://cloud.google.com/compute/docs/images#shielded-images).
        /// **Note**: `allow_stopping_for_update` must be set to true or your instance must have a `desired_status` of `TERMINATED` in order to update this field.
        pub shielded_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceShieldedInstanceConfig,
        >,
        /// A list of network tags to attach to the instance.
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The zone that the machine should be created in. If it is not provided, the provider zone is used.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let advanced_machine_features_binding = args
            .advanced_machine_features
            .get_output(context)
            .get_inner();
        let allow_stopping_for_update_binding = args
            .allow_stopping_for_update
            .get_output(context)
            .get_inner();
        let attached_disks_binding = args.attached_disks.get_output(context).get_inner();
        let boot_disk_binding = args.boot_disk.get_output(context).get_inner();
        let can_ip_forward_binding = args.can_ip_forward.get_output(context).get_inner();
        let confidential_instance_config_binding = args
            .confidential_instance_config
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let desired_status_binding = args.desired_status.get_output(context).get_inner();
        let enable_display_binding = args.enable_display.get_output(context).get_inner();
        let guest_accelerators_binding = args
            .guest_accelerators
            .get_output(context)
            .get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let key_revocation_action_type_binding = args
            .key_revocation_action_type
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let machine_type_binding = args.machine_type.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let metadata_startup_script_binding = args
            .metadata_startup_script
            .get_output(context)
            .get_inner();
        let min_cpu_platform_binding = args
            .min_cpu_platform
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_interfaces_binding = args
            .network_interfaces
            .get_output(context)
            .get_inner();
        let network_performance_config_binding = args
            .network_performance_config
            .get_output(context)
            .get_inner();
        let params_binding = args.params.get_output(context).get_inner();
        let partner_metadata_binding = args
            .partner_metadata
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let reservation_affinity_binding = args
            .reservation_affinity
            .get_output(context)
            .get_inner();
        let resource_policies_binding = args
            .resource_policies
            .get_output(context)
            .get_inner();
        let scheduling_binding = args.scheduling.get_output(context).get_inner();
        let scratch_disks_binding = args.scratch_disks.get_output(context).get_inner();
        let service_account_binding = args
            .service_account
            .get_output(context)
            .get_inner();
        let shielded_instance_config_binding = args
            .shielded_instance_config
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedMachineFeatures".into(),
                    value: &advanced_machine_features_binding,
                },
                register_interface::ObjectField {
                    name: "allowStoppingForUpdate".into(),
                    value: &allow_stopping_for_update_binding,
                },
                register_interface::ObjectField {
                    name: "attachedDisks".into(),
                    value: &attached_disks_binding,
                },
                register_interface::ObjectField {
                    name: "bootDisk".into(),
                    value: &boot_disk_binding,
                },
                register_interface::ObjectField {
                    name: "canIpForward".into(),
                    value: &can_ip_forward_binding,
                },
                register_interface::ObjectField {
                    name: "confidentialInstanceConfig".into(),
                    value: &confidential_instance_config_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "desiredStatus".into(),
                    value: &desired_status_binding,
                },
                register_interface::ObjectField {
                    name: "enableDisplay".into(),
                    value: &enable_display_binding,
                },
                register_interface::ObjectField {
                    name: "guestAccelerators".into(),
                    value: &guest_accelerators_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "keyRevocationActionType".into(),
                    value: &key_revocation_action_type_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "machineType".into(),
                    value: &machine_type_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "metadataStartupScript".into(),
                    value: &metadata_startup_script_binding,
                },
                register_interface::ObjectField {
                    name: "minCpuPlatform".into(),
                    value: &min_cpu_platform_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaces".into(),
                    value: &network_interfaces_binding,
                },
                register_interface::ObjectField {
                    name: "networkPerformanceConfig".into(),
                    value: &network_performance_config_binding,
                },
                register_interface::ObjectField {
                    name: "params".into(),
                    value: &params_binding,
                },
                register_interface::ObjectField {
                    name: "partnerMetadata".into(),
                    value: &partner_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reservationAffinity".into(),
                    value: &reservation_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "resourcePolicies".into(),
                    value: &resource_policies_binding,
                },
                register_interface::ObjectField {
                    name: "scheduling".into(),
                    value: &scheduling_binding,
                },
                register_interface::ObjectField {
                    name: "scratchDisks".into(),
                    value: &scratch_disks_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "shieldedInstanceConfig".into(),
                    value: &shielded_instance_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            advanced_machine_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("advancedMachineFeatures"),
            ),
            allow_stopping_for_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowStoppingForUpdate"),
            ),
            attached_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachedDisks"),
            ),
            boot_disk: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootDisk"),
            ),
            can_ip_forward: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("canIpForward"),
            ),
            confidential_instance_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialInstanceConfig"),
            ),
            cpu_platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cpuPlatform"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            current_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currentStatus"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            desired_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("desiredStatus"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_display: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDisplay"),
            ),
            guest_accelerators: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("guestAccelerators"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            key_revocation_action_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyRevocationActionType"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            machine_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("machineType"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            metadata_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadataFingerprint"),
            ),
            metadata_startup_script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadataStartupScript"),
            ),
            min_cpu_platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minCpuPlatform"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_interfaces: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaces"),
            ),
            network_performance_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkPerformanceConfig"),
            ),
            params: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("params"),
            ),
            partner_metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("partnerMetadata"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reservation_affinity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservationAffinity"),
            ),
            resource_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourcePolicies"),
            ),
            scheduling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scheduling"),
            ),
            scratch_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scratchDisks"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            shielded_instance_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shieldedInstanceConfig"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsFingerprint"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
