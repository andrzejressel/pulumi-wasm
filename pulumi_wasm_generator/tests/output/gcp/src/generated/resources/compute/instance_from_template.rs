/// Manages a VM instance resource within GCE. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/instances)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/instances).
///
/// This resource is specifically to create a compute instance from a given
/// `source_instance_template`. To create an instance without a template, use the
/// `gcp.compute.Instance` resource.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   tpl:
///     type: gcp:compute:InstanceTemplate
///     properties:
///       name: template
///       machineType: e2-medium
///       disks:
///         - sourceImage: debian-cloud/debian-11
///           autoDelete: true
///           diskSizeGb: 100
///           boot: true
///       networkInterfaces:
///         - network: default
///       metadata:
///         foo: bar
///       canIpForward: true
///   tplInstanceFromTemplate:
///     type: gcp:compute:InstanceFromTemplate
///     name: tpl
///     properties:
///       name: instance-from-template
///       zone: us-central1-a
///       sourceInstanceTemplate: ${tpl.selfLinkUnique}
///       canIpForward: false
///       labels:
///         my_key: my_value
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod instance_from_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceFromTemplateArgs {
        /// Controls for advanced machine-related behavior features.
        #[builder(into, default)]
        pub advanced_machine_features: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::InstanceFromTemplateAdvancedMachineFeatures,
            >,
        >,
        #[builder(into, default)]
        pub allow_stopping_for_update: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of disks attached to the instance
        #[builder(into, default)]
        pub attached_disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceFromTemplateAttachedDisk>>,
        >,
        /// The boot disk for the instance.
        #[builder(into, default)]
        pub boot_disk: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceFromTemplateBootDisk>,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        #[builder(into, default)]
        pub can_ip_forward: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        #[builder(into, default)]
        pub confidential_instance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::InstanceFromTemplateConfidentialInstanceConfig,
            >,
        >,
        /// Whether deletion protection is enabled on this instance.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// A brief description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Desired status of the instance. Either "RUNNING", "SUSPENDED" or "TERMINATED".
        #[builder(into, default)]
        pub desired_status: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the instance has virtual displays enabled.
        #[builder(into, default)]
        pub enable_display: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance.
        #[builder(into, default)]
        pub guest_accelerators: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceFromTemplateGuestAccelerator>,
            >,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of
        /// labels 1-63 characters long matching the regular expression a-z, concatenated with periods. The entire hostname must not
        /// exceed 253 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// Action to be taken when a customer's encryption key is revoked. Supports "STOP" and "NONE", with "NONE" being the
        /// default.
        #[builder(into, default)]
        pub key_revocation_action_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of key/value label pairs assigned to the instance. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        #[builder(into, default)]
        pub machine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Metadata key/value pairs made available within the instance.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Metadata startup scripts made available within the instance.
        #[builder(into, default)]
        pub metadata_startup_script: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum CPU platform specified for the VM instance.
        #[builder(into, default)]
        pub min_cpu_platform: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks attached to the instance.
        #[builder(into, default)]
        pub network_interfaces: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::compute::InstanceFromTemplateNetworkInterface>,
            >,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        #[builder(into, default)]
        pub network_performance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::InstanceFromTemplateNetworkPerformanceConfig,
            >,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        #[builder(into, default)]
        pub params: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceFromTemplateParams>,
        >,
        /// Partner Metadata Map made available within the instance.
        #[builder(into, default)]
        pub partner_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither
        /// self_link nor project are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the reservations that this instance can consume from.
        #[builder(into, default)]
        pub reservation_affinity: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceFromTemplateReservationAffinity>,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        #[builder(into, default)]
        pub resource_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// The scheduling strategy being used by the instance.
        #[builder(into, default)]
        pub scheduling: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceFromTemplateScheduling>,
        >,
        /// The scratch disks attached to the instance.
        #[builder(into, default)]
        pub scratch_disks: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::InstanceFromTemplateScratchDisk>>,
        >,
        /// The service account to attach to the instance.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::InstanceFromTemplateServiceAccount>,
        >,
        /// The shielded vm config being used by the instance.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::compute::InstanceFromTemplateShieldedInstanceConfig,
            >,
        >,
        /// Name or self link of an instance
        /// template to create the instance based on. It is recommended to reference
        /// instance templates through their unique id (`self_link_unique` attribute).
        ///
        /// - - -
        #[builder(into)]
        pub source_instance_template: pulumi_wasm_rust::Output<String>,
        /// The list of tags attached to the instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, all arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the template. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceFromTemplateResult {
        /// Controls for advanced machine-related behavior features.
        pub advanced_machine_features: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateAdvancedMachineFeatures,
        >,
        pub allow_stopping_for_update: pulumi_wasm_rust::Output<bool>,
        /// List of disks attached to the instance
        pub attached_disks: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateAttachedDisk>,
        >,
        /// The boot disk for the instance.
        pub boot_disk: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateBootDisk,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        pub can_ip_forward: pulumi_wasm_rust::Output<bool>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        pub confidential_instance_config: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateConfidentialInstanceConfig,
        >,
        /// The CPU platform used by this instance.
        pub cpu_platform: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// Current status of the instance. This could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING,
        /// SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see [Instance
        /// life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle).
        pub current_status: pulumi_wasm_rust::Output<String>,
        /// Whether deletion protection is enabled on this instance.
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        /// A brief description of the resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Desired status of the instance. Either "RUNNING", "SUSPENDED" or "TERMINATED".
        pub desired_status: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the instance has virtual displays enabled.
        pub enable_display: pulumi_wasm_rust::Output<bool>,
        /// List of the type and count of accelerator cards attached to the instance.
        pub guest_accelerators: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateGuestAccelerator>,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of
        /// labels 1-63 characters long matching the regular expression a-z, concatenated with periods. The entire hostname must not
        /// exceed 253 characters. Changing this forces a new resource to be created.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The server-assigned unique identifier of this instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked. Supports "STOP" and "NONE", with "NONE" being the
        /// default.
        pub key_revocation_action_type: pulumi_wasm_rust::Output<String>,
        /// The unique fingerprint of the labels.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// A set of key/value label pairs assigned to the instance. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The machine type to create.
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Metadata key/value pairs made available within the instance.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Metadata startup scripts made available within the instance.
        pub metadata_startup_script: pulumi_wasm_rust::Output<String>,
        /// The minimum CPU platform specified for the VM instance.
        pub min_cpu_platform: pulumi_wasm_rust::Output<String>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The networks attached to the instance.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateNetworkInterface>,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        pub network_performance_config: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateNetworkPerformanceConfig,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        pub params: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateParams,
        >,
        /// Partner Metadata Map made available within the instance.
        pub partner_metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither
        /// self_link nor project are provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the reservations that this instance can consume from.
        pub reservation_affinity: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateReservationAffinity,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_wasm_rust::Output<String>,
        /// The scheduling strategy being used by the instance.
        pub scheduling: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateScheduling,
        >,
        /// The scratch disks attached to the instance.
        pub scratch_disks: pulumi_wasm_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateScratchDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The service account to attach to the instance.
        pub service_account: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateServiceAccount,
        >,
        /// The shielded vm config being used by the instance.
        pub shielded_instance_config: pulumi_wasm_rust::Output<
            super::super::types::compute::InstanceFromTemplateShieldedInstanceConfig,
        >,
        /// Name or self link of an instance
        /// template to create the instance based on. It is recommended to reference
        /// instance templates through their unique id (`self_link_unique` attribute).
        ///
        /// - - -
        pub source_instance_template: pulumi_wasm_rust::Output<String>,
        /// The list of tags attached to the instance.
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_wasm_rust::Output<String>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, all arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the template. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceFromTemplateArgs,
    ) -> InstanceFromTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let advanced_machine_features_binding = args
            .advanced_machine_features
            .get_inner();
        let allow_stopping_for_update_binding = args
            .allow_stopping_for_update
            .get_inner();
        let attached_disks_binding = args.attached_disks.get_inner();
        let boot_disk_binding = args.boot_disk.get_inner();
        let can_ip_forward_binding = args.can_ip_forward.get_inner();
        let confidential_instance_config_binding = args
            .confidential_instance_config
            .get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let description_binding = args.description.get_inner();
        let desired_status_binding = args.desired_status.get_inner();
        let enable_display_binding = args.enable_display.get_inner();
        let guest_accelerators_binding = args.guest_accelerators.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let key_revocation_action_type_binding = args
            .key_revocation_action_type
            .get_inner();
        let labels_binding = args.labels.get_inner();
        let machine_type_binding = args.machine_type.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let metadata_startup_script_binding = args.metadata_startup_script.get_inner();
        let min_cpu_platform_binding = args.min_cpu_platform.get_inner();
        let name_binding = args.name.get_inner();
        let network_interfaces_binding = args.network_interfaces.get_inner();
        let network_performance_config_binding = args
            .network_performance_config
            .get_inner();
        let params_binding = args.params.get_inner();
        let partner_metadata_binding = args.partner_metadata.get_inner();
        let project_binding = args.project.get_inner();
        let reservation_affinity_binding = args.reservation_affinity.get_inner();
        let resource_policies_binding = args.resource_policies.get_inner();
        let scheduling_binding = args.scheduling.get_inner();
        let scratch_disks_binding = args.scratch_disks.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let shielded_instance_config_binding = args.shielded_instance_config.get_inner();
        let source_instance_template_binding = args.source_instance_template.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceFromTemplate:InstanceFromTemplate".into(),
            name: name.to_string(),
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
                    name: "sourceInstanceTemplate".into(),
                    value: &source_instance_template_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "advancedMachineFeatures".into(),
                },
                register_interface::ResultField {
                    name: "allowStoppingForUpdate".into(),
                },
                register_interface::ResultField {
                    name: "attachedDisks".into(),
                },
                register_interface::ResultField {
                    name: "bootDisk".into(),
                },
                register_interface::ResultField {
                    name: "canIpForward".into(),
                },
                register_interface::ResultField {
                    name: "confidentialInstanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "cpuPlatform".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "currentStatus".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "desiredStatus".into(),
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
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "keyRevocationActionType".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
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
                    name: "networkInterfaces".into(),
                },
                register_interface::ResultField {
                    name: "networkPerformanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "params".into(),
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
                    name: "reservationAffinity".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicies".into(),
                },
                register_interface::ResultField {
                    name: "scheduling".into(),
                },
                register_interface::ResultField {
                    name: "scratchDisks".into(),
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
                    name: "sourceInstanceTemplate".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceFromTemplateResult {
            advanced_machine_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedMachineFeatures").unwrap(),
            ),
            allow_stopping_for_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowStoppingForUpdate").unwrap(),
            ),
            attached_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedDisks").unwrap(),
            ),
            boot_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDisk").unwrap(),
            ),
            can_ip_forward: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canIpForward").unwrap(),
            ),
            confidential_instance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialInstanceConfig").unwrap(),
            ),
            cpu_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuPlatform").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            current_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentStatus").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            desired_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredStatus").unwrap(),
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
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            key_revocation_action_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRevocationActionType").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
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
            network_interfaces: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaces").unwrap(),
            ),
            network_performance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkPerformanceConfig").unwrap(),
            ),
            params: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("params").unwrap(),
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
            reservation_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationAffinity").unwrap(),
            ),
            resource_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicies").unwrap(),
            ),
            scheduling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduling").unwrap(),
            ),
            scratch_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scratchDisks").unwrap(),
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
            source_instance_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceInstanceTemplate").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsFingerprint").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
