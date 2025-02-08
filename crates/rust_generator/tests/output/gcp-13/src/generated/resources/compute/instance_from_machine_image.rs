/// Manages a VM instance resource within GCE. For more information see
/// [the official documentation](https://cloud.google.com/compute/docs/instances)
/// and
/// [API](https://cloud.google.com/compute/docs/reference/latest/instances).
///
/// This resource is specifically to create a compute instance from a given
/// `source_machine_image`. To create an instance without a machine image, use the
/// `gcp.compute.Instance` resource.
///
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   tpl:
///     type: gcp:compute:InstanceFromMachineImage
///     properties:
///       name: instance-from-machine-image
///       zone: us-central1-a
///       sourceMachineImage: projects/PROJECT-ID/global/machineImages/NAME
///       canIpForward: false
///       labels:
///         my_key: my_value
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod instance_from_machine_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceFromMachineImageArgs {
        /// Controls for advanced machine-related behavior features.
        #[builder(into, default)]
        pub advanced_machine_features: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromMachineImageAdvancedMachineFeatures,
            >,
        >,
        #[builder(into, default)]
        pub allow_stopping_for_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        #[builder(into, default)]
        pub can_ip_forward: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        #[builder(into, default)]
        pub confidential_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromMachineImageConfidentialInstanceConfig,
            >,
        >,
        /// Whether deletion protection is enabled on this instance.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A brief description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Desired status of the instance. Either "RUNNING", "SUSPENDED" or "TERMINATED".
        #[builder(into, default)]
        pub desired_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the instance has virtual displays enabled.
        #[builder(into, default)]
        pub enable_display: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of the type and count of accelerator cards attached to the instance.
        #[builder(into, default)]
        pub guest_accelerators: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::InstanceFromMachineImageGuestAccelerator,
                >,
            >,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of
        /// labels 1-63 characters long matching the regular expression a-z, concatenated with periods. The entire hostname must not
        /// exceed 253 characters. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Action to be taken when a customer's encryption key is revoked. Supports "STOP" and "NONE", with "NONE" being the
        /// default.
        #[builder(into, default)]
        pub key_revocation_action_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A set of key/value label pairs assigned to the instance. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to create.
        #[builder(into, default)]
        pub machine_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Metadata key/value pairs made available within the instance.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Metadata startup scripts made available within the instance.
        #[builder(into, default)]
        pub metadata_startup_script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum CPU platform specified for the VM instance.
        #[builder(into, default)]
        pub min_cpu_platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The networks attached to the instance.
        #[builder(into, default)]
        pub network_interfaces: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::compute::InstanceFromMachineImageNetworkInterface,
                >,
            >,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        #[builder(into, default)]
        pub network_performance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromMachineImageNetworkPerformanceConfig,
            >,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        #[builder(into, default)]
        pub params: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromMachineImageParams>,
        >,
        /// Partner Metadata Map made available within the instance.
        #[builder(into, default)]
        pub partner_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither
        /// self_link nor project are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the reservations that this instance can consume from.
        #[builder(into, default)]
        pub reservation_affinity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromMachineImageReservationAffinity,
            >,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        #[builder(into, default)]
        pub resource_policies: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scheduling strategy being used by the instance.
        #[builder(into, default)]
        pub scheduling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromMachineImageScheduling>,
        >,
        /// The service account to attach to the instance.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromMachineImageServiceAccount>,
        >,
        /// The shielded vm config being used by the instance.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromMachineImageShieldedInstanceConfig,
            >,
        >,
        /// Name or self link of a machine
        /// image to create the instance based on.
        ///
        /// - - -
        #[builder(into)]
        pub source_machine_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of tags attached to the instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, most* arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the machine image. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        ///
        /// > **Warning:** *Due to API limitations, disk overrides are currently disabled. This includes the "boot_disk", "attached_disk", and "scratch_disk" fields.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceFromMachineImageResult {
        /// Controls for advanced machine-related behavior features.
        pub advanced_machine_features: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageAdvancedMachineFeatures,
        >,
        pub allow_stopping_for_update: pulumi_gestalt_rust::Output<bool>,
        /// List of disks attached to the instance
        pub attached_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromMachineImageAttachedDisk>,
        >,
        /// The boot disk for the instance.
        pub boot_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromMachineImageBootDisk>,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        pub can_ip_forward: pulumi_gestalt_rust::Output<bool>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        pub confidential_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageConfidentialInstanceConfig,
        >,
        /// The CPU platform used by this instance.
        pub cpu_platform: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Current status of the instance. This could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING,
        /// SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see [Instance
        /// life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle).
        pub current_status: pulumi_gestalt_rust::Output<String>,
        /// Whether deletion protection is enabled on this instance.
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        /// A brief description of the resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Desired status of the instance. Either "RUNNING", "SUSPENDED" or "TERMINATED".
        pub desired_status: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the instance has virtual displays enabled.
        pub enable_display: pulumi_gestalt_rust::Output<bool>,
        /// List of the type and count of accelerator cards attached to the instance.
        pub guest_accelerators: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromMachineImageGuestAccelerator>,
        >,
        /// A custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of
        /// labels 1-63 characters long matching the regular expression a-z, concatenated with periods. The entire hostname must not
        /// exceed 253 characters. Changing this forces a new resource to be created.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The server-assigned unique identifier of this instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked. Supports "STOP" and "NONE", with "NONE" being the
        /// default.
        pub key_revocation_action_type: pulumi_gestalt_rust::Output<String>,
        /// The unique fingerprint of the labels.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// A set of key/value label pairs assigned to the instance. **Note**: This field is non-authoritative, and will only manage
        /// the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on
        /// the resource.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The machine type to create.
        pub machine_type: pulumi_gestalt_rust::Output<String>,
        /// Metadata key/value pairs made available within the instance.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Metadata startup scripts made available within the instance.
        pub metadata_startup_script: pulumi_gestalt_rust::Output<String>,
        /// The minimum CPU platform specified for the VM instance.
        pub min_cpu_platform: pulumi_gestalt_rust::Output<String>,
        /// A unique name for the resource, required by GCE.
        /// Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The networks attached to the instance.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromMachineImageNetworkInterface>,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        pub network_performance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageNetworkPerformanceConfig,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        pub params: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageParams,
        >,
        /// Partner Metadata Map made available within the instance.
        pub partner_metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither
        /// self_link nor project are provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the reservations that this instance can consume from.
        pub reservation_affinity: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageReservationAffinity,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_gestalt_rust::Output<String>,
        /// The scheduling strategy being used by the instance.
        pub scheduling: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageScheduling,
        >,
        /// The scratch disks attached to the instance.
        pub scratch_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromMachineImageScratchDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The service account to attach to the instance.
        pub service_account: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageServiceAccount,
        >,
        /// The shielded vm config being used by the instance.
        pub shielded_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromMachineImageShieldedInstanceConfig,
        >,
        /// Name or self link of a machine
        /// image to create the instance based on.
        ///
        /// - - -
        pub source_machine_image: pulumi_gestalt_rust::Output<String>,
        /// The list of tags attached to the instance.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, most* arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the machine image. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        ///
        /// > **Warning:** *Due to API limitations, disk overrides are currently disabled. This includes the "boot_disk", "attached_disk", and "scratch_disk" fields.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceFromMachineImageArgs,
    ) -> InstanceFromMachineImageResult {
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
        let service_account_binding = args
            .service_account
            .get_output(context)
            .get_inner();
        let shielded_instance_config_binding = args
            .shielded_instance_config
            .get_output(context)
            .get_inner();
        let source_machine_image_binding = args
            .source_machine_image
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/instanceFromMachineImage:InstanceFromMachineImage"
                .into(),
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
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "shieldedInstanceConfig".into(),
                    value: &shielded_instance_config_binding,
                },
                register_interface::ObjectField {
                    name: "sourceMachineImage".into(),
                    value: &source_machine_image_binding,
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
        InstanceFromMachineImageResult {
            advanced_machine_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("advancedMachineFeatures"),
            ),
            allow_stopping_for_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowStoppingForUpdate"),
            ),
            attached_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachedDisks"),
            ),
            boot_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bootDisks"),
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
            source_machine_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceMachineImage"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsFingerprint"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
