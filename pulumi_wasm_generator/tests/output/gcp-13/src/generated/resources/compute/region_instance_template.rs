///
///
/// ## Import
///
/// Instance templates can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/instanceTemplates/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, instance templates can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionInstanceTemplate:RegionInstanceTemplate default projects/{{project}}/regions/{{region}}/instanceTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionInstanceTemplate:RegionInstanceTemplate default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionInstanceTemplate:RegionInstanceTemplate default {{name}}
/// ```
///
pub mod region_instance_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionInstanceTemplateArgs {
        /// Configure Nested Virtualisation and Simultaneous Hyper Threading on this VM. Structure is documented below
        #[builder(into, default)]
        pub advanced_machine_features: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateAdvancedMachineFeatures,
            >,
        >,
        /// Whether to allow sending and receiving of
        /// packets with non-matching source or destination IPs. This defaults to false.
        #[builder(into, default)]
        pub can_ip_forward: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable [Confidential Mode](https://cloud.google.com/compute/confidential-vm/docs/about-cvm) on this VM. Structure is documented below
        #[builder(into, default)]
        pub confidential_instance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateConfidentialInstanceConfig,
            >,
        >,
        /// A brief description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Disks to attach to instances created from this template.
        /// This can be specified multiple times for multiple disks. Structure is
        /// documented below.
        #[builder(into)]
        pub disks: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::RegionInstanceTemplateDisk>,
        >,
        /// Enable [Virtual Displays](https://cloud.google.com/compute/docs/instances/enable-instance-virtual-display#verify_display_driver) on this instance.
        /// **Note**: `allow_stopping_for_update` must be set to true in order to update this field.
        #[builder(into, default)]
        pub enable_display: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance. Structure documented below.
        #[builder(into, default)]
        pub guest_accelerators: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::RegionInstanceTemplateGuestAccelerator>,
            >,
        >,
        /// A brief description to use for instances
        /// created from this template.
        #[builder(into, default)]
        pub instance_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Action to be taken when a customer's encryption key is revoked. Supports `STOP` and `NONE`, with `NONE` being the default.
        #[builder(into, default)]
        pub key_revocation_action_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to instances
        /// created from this template.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        ///
        /// To create a machine with a [custom type](https://cloud.google.com/dataproc/docs/concepts/compute/custom-machine-types) (such as extended memory), format the value like `custom-VCPUS-MEM_IN_MB` like `custom-6-20480` for 6 vCPU and 20GB of RAM.
        ///
        /// - - -
        #[builder(into)]
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Metadata key/value pairs to make available from
        /// within instances created from this template.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An alternative to using the
        /// startup-script metadata key, mostly to match the compute_instance resource.
        /// This replaces the startup-script metadata key on the created instance and
        /// thus the two mechanisms are not allowed to be used simultaneously.
        #[builder(into, default)]
        pub metadata_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as
        /// `Intel Haswell` or `Intel Skylake`. See the complete list [here](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform).
        #[builder(into, default)]
        pub min_cpu_platform: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        ///
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Networks to attach to instances created from
        /// this template. This can be specified multiple times for multiple networks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::RegionInstanceTemplateNetworkInterface>,
            >,
        >,
        /// Configures network performance settings for the instance created from the
        /// template. Structure is documented below. **Note**: `machine_type`
        /// must be a [supported type](https://cloud.google.com/compute/docs/networking/configure-vm-with-high-bandwidth-configuration),
        /// the `image` used must include the [`GVNIC`](https://cloud.google.com/compute/docs/networking/using-gvnic#create-instance-gvnic-image)
        /// in `guest-os-features`, and `network_interface.0.nic-type` must be `GVNIC`
        /// in order for this setting to take effect.
        #[builder(into, default)]
        pub network_performance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateNetworkPerformanceConfig,
            >,
        >,
        /// Beta key/value pair represents partner metadata assigned to instance template where key represent a defined namespace and value is a json string represent the entries associted with the namespace.
        #[builder(into, default)]
        pub partner_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Region in which the resource belongs.
        /// If region is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the reservations that this instance can consume from.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reservation_affinity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateReservationAffinity,
            >,
        >,
        /// A set of key/value resource manager tag pairs to bind to the instance. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.
        #[builder(into, default)]
        pub resource_manager_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// - A list of self_links of resource policies to attach to the instance. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
        #[builder(into, default)]
        pub resource_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// The scheduling strategy to use. More details about
        /// this configuration option are detailed below.
        #[builder(into, default)]
        pub scheduling: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionInstanceTemplateScheduling>,
        >,
        /// Service account to attach to the instance. Structure is documented below.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionInstanceTemplateServiceAccount>,
        >,
        /// Enable [Shielded VM](https://cloud.google.com/security/shielded-cloud/shielded-vm) on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Structure is documented below.
        /// **Note**: `shielded_instance_config` can only be used with boot images with shielded vm support. See the complete list [here](https://cloud.google.com/compute/docs/images#shielded-images).
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateShieldedInstanceConfig,
            >,
        >,
        /// Tags to attach to the instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct RegionInstanceTemplateResult {
        /// Configure Nested Virtualisation and Simultaneous Hyper Threading on this VM. Structure is documented below
        pub advanced_machine_features: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateAdvancedMachineFeatures,
            >,
        >,
        /// Whether to allow sending and receiving of
        /// packets with non-matching source or destination IPs. This defaults to false.
        pub can_ip_forward: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable [Confidential Mode](https://cloud.google.com/compute/confidential-vm/docs/about-cvm) on this VM. Structure is documented below
        pub confidential_instance_config: pulumi_wasm_rust::Output<
            super::super::types::compute::RegionInstanceTemplateConfidentialInstanceConfig,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// A brief description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Disks to attach to instances created from this template.
        /// This can be specified multiple times for multiple disks. Structure is
        /// documented below.
        pub disks: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::RegionInstanceTemplateDisk>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable [Virtual Displays](https://cloud.google.com/compute/docs/instances/enable-instance-virtual-display#verify_display_driver) on this instance.
        /// **Note**: `allow_stopping_for_update` must be set to true in order to update this field.
        pub enable_display: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance. Structure documented below.
        pub guest_accelerators: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::RegionInstanceTemplateGuestAccelerator>,
            >,
        >,
        /// A brief description to use for instances
        /// created from this template.
        pub instance_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Action to be taken when a customer's encryption key is revoked. Supports `STOP` and `NONE`, with `NONE` being the default.
        pub key_revocation_action_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to instances
        /// created from this template.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        ///
        /// To create a machine with a [custom type](https://cloud.google.com/dataproc/docs/concepts/compute/custom-machine-types) (such as extended memory), format the value like `custom-VCPUS-MEM_IN_MB` like `custom-6-20480` for 6 vCPU and 20GB of RAM.
        ///
        /// - - -
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Metadata key/value pairs to make available from
        /// within instances created from this template.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_wasm_rust::Output<String>,
        /// An alternative to using the
        /// startup-script metadata key, mostly to match the compute_instance resource.
        /// This replaces the startup-script metadata key on the created instance and
        /// thus the two mechanisms are not allowed to be used simultaneously.
        pub metadata_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as
        /// `Intel Haswell` or `Intel Skylake`. See the complete list [here](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform).
        pub min_cpu_platform: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        ///
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Networks to attach to instances created from
        /// this template. This can be specified multiple times for multiple networks.
        /// Structure is documented below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::RegionInstanceTemplateNetworkInterface>,
            >,
        >,
        /// Configures network performance settings for the instance created from the
        /// template. Structure is documented below. **Note**: `machine_type`
        /// must be a [supported type](https://cloud.google.com/compute/docs/networking/configure-vm-with-high-bandwidth-configuration),
        /// the `image` used must include the [`GVNIC`](https://cloud.google.com/compute/docs/networking/using-gvnic#create-instance-gvnic-image)
        /// in `guest-os-features`, and `network_interface.0.nic-type` must be `GVNIC`
        /// in order for this setting to take effect.
        pub network_performance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateNetworkPerformanceConfig,
            >,
        >,
        /// Beta key/value pair represents partner metadata assigned to instance template where key represent a defined namespace and value is a json string represent the entries associted with the namespace.
        pub partner_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Region in which the resource belongs.
        /// If region is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Specifies the reservations that this instance can consume from.
        /// Structure is documented below.
        pub reservation_affinity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::RegionInstanceTemplateReservationAffinity,
            >,
        >,
        /// A set of key/value resource manager tag pairs to bind to the instance. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456.
        pub resource_manager_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// - A list of self_links of resource policies to attach to the instance. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// The scheduling strategy to use. More details about
        /// this configuration option are detailed below.
        pub scheduling: pulumi_wasm_rust::Output<
            super::super::types::compute::RegionInstanceTemplateScheduling,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Service account to attach to the instance. Structure is documented below.
        pub service_account: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionInstanceTemplateServiceAccount>,
        >,
        /// Enable [Shielded VM](https://cloud.google.com/security/shielded-cloud/shielded-vm) on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Structure is documented below.
        /// **Note**: `shielded_instance_config` can only be used with boot images with shielded vm support. See the complete list [here](https://cloud.google.com/compute/docs/images#shielded-images).
        pub shielded_instance_config: pulumi_wasm_rust::Output<
            super::super::types::compute::RegionInstanceTemplateShieldedInstanceConfig,
        >,
        /// Tags to attach to the instance.
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RegionInstanceTemplateArgs,
    ) -> RegionInstanceTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_machine_features_binding = args
            .advanced_machine_features
            .get_inner();
        let can_ip_forward_binding = args.can_ip_forward.get_inner();
        let confidential_instance_config_binding = args
            .confidential_instance_config
            .get_inner();
        let description_binding = args.description.get_inner();
        let disks_binding = args.disks.get_inner();
        let enable_display_binding = args.enable_display.get_inner();
        let guest_accelerators_binding = args.guest_accelerators.get_inner();
        let instance_description_binding = args.instance_description.get_inner();
        let key_revocation_action_type_binding = args
            .key_revocation_action_type
            .get_inner();
        let labels_binding = args.labels.get_inner();
        let machine_type_binding = args.machine_type.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let metadata_startup_script_binding = args.metadata_startup_script.get_inner();
        let min_cpu_platform_binding = args.min_cpu_platform.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let network_interfaces_binding = args.network_interfaces.get_inner();
        let network_performance_config_binding = args
            .network_performance_config
            .get_inner();
        let partner_metadata_binding = args.partner_metadata.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let reservation_affinity_binding = args.reservation_affinity.get_inner();
        let resource_manager_tags_binding = args.resource_manager_tags.get_inner();
        let resource_policies_binding = args.resource_policies.get_inner();
        let scheduling_binding = args.scheduling.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let shielded_instance_config_binding = args.shielded_instance_config.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionInstanceTemplate:RegionInstanceTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "advancedMachineFeatures".into(),
                    value: &advanced_machine_features_binding,
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
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disks".into(),
                    value: &disks_binding,
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
                    name: "instanceDescription".into(),
                    value: &instance_description_binding,
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
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
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
                    name: "partnerMetadata".into(),
                    value: &partner_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "reservationAffinity".into(),
                    value: &reservation_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "resourceManagerTags".into(),
                    value: &resource_manager_tags_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "advancedMachineFeatures".into(),
                },
                register_interface::ResultField {
                    name: "canIpForward".into(),
                },
                register_interface::ResultField {
                    name: "confidentialInstanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disks".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableDisplay".into(),
                },
                register_interface::ResultField {
                    name: "guestAccelerators".into(),
                },
                register_interface::ResultField {
                    name: "instanceDescription".into(),
                },
                register_interface::ResultField {
                    name: "keyRevocationActionType".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "machineType".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "metadataFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "metadataStartupScript".into(),
                },
                register_interface::ResultField {
                    name: "minCpuPlatform".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "networkPerformanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "partnerMetadata".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "reservationAffinity".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagerTags".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicies".into(),
                },
                register_interface::ResultField {
                    name: "scheduling".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "shieldedInstanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsFingerprint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionInstanceTemplateResult {
            advanced_machine_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedMachineFeatures").unwrap(),
            ),
            can_ip_forward: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canIpForward").unwrap(),
            ),
            confidential_instance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialInstanceConfig").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disks").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_display: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDisplay").unwrap(),
            ),
            guest_accelerators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guestAccelerators").unwrap(),
            ),
            instance_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceDescription").unwrap(),
            ),
            key_revocation_action_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRevocationActionType").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            machine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineType").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            metadata_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataFingerprint").unwrap(),
            ),
            metadata_startup_script: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataStartupScript").unwrap(),
            ),
            min_cpu_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minCpuPlatform").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            network_performance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkPerformanceConfig").unwrap(),
            ),
            partner_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerMetadata").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            reservation_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationAffinity").unwrap(),
            ),
            resource_manager_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerTags").unwrap(),
            ),
            resource_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicies").unwrap(),
            ),
            scheduling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduling").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            shielded_instance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shieldedInstanceConfig").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsFingerprint").unwrap(),
            ),
        }
    }
}
