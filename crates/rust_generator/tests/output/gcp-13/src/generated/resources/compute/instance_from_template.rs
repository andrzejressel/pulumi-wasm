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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_from_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceFromTemplateArgs {
        /// Controls for advanced machine-related behavior features.
        #[builder(into, default)]
        pub advanced_machine_features: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromTemplateAdvancedMachineFeatures,
            >,
        >,
        #[builder(into, default)]
        pub allow_stopping_for_update: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of disks attached to the instance
        #[builder(into, default)]
        pub attached_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceFromTemplateAttachedDisk>>,
        >,
        /// The boot disk for the instance.
        #[builder(into, default)]
        pub boot_disk: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromTemplateBootDisk>,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        #[builder(into, default)]
        pub can_ip_forward: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        #[builder(into, default)]
        pub confidential_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromTemplateConfidentialInstanceConfig,
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
                Vec<super::super::types::compute::InstanceFromTemplateGuestAccelerator>,
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
                Vec<super::super::types::compute::InstanceFromTemplateNetworkInterface>,
            >,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        #[builder(into, default)]
        pub network_performance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::InstanceFromTemplateNetworkPerformanceConfig,
            >,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        #[builder(into, default)]
        pub params: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromTemplateParams>,
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
            Option<super::super::types::compute::InstanceFromTemplateReservationAffinity>,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        #[builder(into, default)]
        pub resource_policies: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scheduling strategy being used by the instance.
        #[builder(into, default)]
        pub scheduling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromTemplateScheduling>,
        >,
        /// The scratch disks attached to the instance.
        #[builder(into, default)]
        pub scratch_disks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::InstanceFromTemplateScratchDisk>>,
        >,
        /// The service account to attach to the instance.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::InstanceFromTemplateServiceAccount>,
        >,
        /// The shielded vm config being used by the instance.
        #[builder(into, default)]
        pub shielded_instance_config: pulumi_gestalt_rust::InputOrOutput<
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
        pub source_instance_template: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of tags attached to the instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, all arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the template. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceFromTemplateResult {
        /// Controls for advanced machine-related behavior features.
        pub advanced_machine_features: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateAdvancedMachineFeatures,
        >,
        pub allow_stopping_for_update: pulumi_gestalt_rust::Output<bool>,
        /// List of disks attached to the instance
        pub attached_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateAttachedDisk>,
        >,
        /// The boot disk for the instance.
        pub boot_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateBootDisk,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        pub can_ip_forward: pulumi_gestalt_rust::Output<bool>,
        /// The Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail
        /// to create.
        pub confidential_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateConfidentialInstanceConfig,
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
            Vec<super::super::types::compute::InstanceFromTemplateGuestAccelerator>,
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
            Vec<super::super::types::compute::InstanceFromTemplateNetworkInterface>,
        >,
        /// Configures network performance settings for the instance. If not specified, the instance will be created with its
        /// default network performance configuration.
        pub network_performance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateNetworkPerformanceConfig,
        >,
        /// Stores additional params passed with the request, but not persisted as part of resource payload.
        pub params: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateParams,
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
            super::super::types::compute::InstanceFromTemplateReservationAffinity,
        >,
        /// A list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_gestalt_rust::Output<String>,
        /// The scheduling strategy being used by the instance.
        pub scheduling: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateScheduling,
        >,
        /// The scratch disks attached to the instance.
        pub scratch_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::compute::InstanceFromTemplateScratchDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The service account to attach to the instance.
        pub service_account: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateServiceAccount,
        >,
        /// The shielded vm config being used by the instance.
        pub shielded_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::InstanceFromTemplateShieldedInstanceConfig,
        >,
        /// Name or self link of an instance
        /// template to create the instance based on. It is recommended to reference
        /// instance templates through their unique id (`self_link_unique` attribute).
        ///
        /// - - -
        pub source_instance_template: pulumi_gestalt_rust::Output<String>,
        /// The list of tags attached to the instance.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The zone that the machine should be created in. If not
        /// set, the provider zone is used.
        ///
        /// In addition to these, all arguments from `gcp.compute.Instance` are supported
        /// as a way to override the properties in the template. All exported attributes
        /// from `gcp.compute.Instance` are likewise exported here.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceFromTemplateArgs,
    ) -> InstanceFromTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let advanced_machine_features_binding = args
            .advanced_machine_features
            .get_output(context);
        let allow_stopping_for_update_binding = args
            .allow_stopping_for_update
            .get_output(context);
        let attached_disks_binding = args.attached_disks.get_output(context);
        let boot_disk_binding = args.boot_disk.get_output(context);
        let can_ip_forward_binding = args.can_ip_forward.get_output(context);
        let confidential_instance_config_binding = args
            .confidential_instance_config
            .get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let description_binding = args.description.get_output(context);
        let desired_status_binding = args.desired_status.get_output(context);
        let enable_display_binding = args.enable_display.get_output(context);
        let guest_accelerators_binding = args.guest_accelerators.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let key_revocation_action_type_binding = args
            .key_revocation_action_type
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let machine_type_binding = args.machine_type.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let metadata_startup_script_binding = args
            .metadata_startup_script
            .get_output(context);
        let min_cpu_platform_binding = args.min_cpu_platform.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_interfaces_binding = args.network_interfaces.get_output(context);
        let network_performance_config_binding = args
            .network_performance_config
            .get_output(context);
        let params_binding = args.params.get_output(context);
        let partner_metadata_binding = args.partner_metadata.get_output(context);
        let project_binding = args.project.get_output(context);
        let reservation_affinity_binding = args.reservation_affinity.get_output(context);
        let resource_policies_binding = args.resource_policies.get_output(context);
        let scheduling_binding = args.scheduling.get_output(context);
        let scratch_disks_binding = args.scratch_disks.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let shielded_instance_config_binding = args
            .shielded_instance_config
            .get_output(context);
        let source_instance_template_binding = args
            .source_instance_template
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/instanceFromTemplate:InstanceFromTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "advancedMachineFeatures".into(),
                    value: advanced_machine_features_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowStoppingForUpdate".into(),
                    value: allow_stopping_for_update_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachedDisks".into(),
                    value: attached_disks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bootDisk".into(),
                    value: boot_disk_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canIpForward".into(),
                    value: can_ip_forward_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confidentialInstanceConfig".into(),
                    value: confidential_instance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: deletion_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desiredStatus".into(),
                    value: desired_status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableDisplay".into(),
                    value: enable_display_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guestAccelerators".into(),
                    value: guest_accelerators_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRevocationActionType".into(),
                    value: key_revocation_action_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineType".into(),
                    value: machine_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataStartupScript".into(),
                    value: metadata_startup_script_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minCpuPlatform".into(),
                    value: min_cpu_platform_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkInterfaces".into(),
                    value: network_interfaces_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkPerformanceConfig".into(),
                    value: network_performance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "params".into(),
                    value: params_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerMetadata".into(),
                    value: partner_metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservationAffinity".into(),
                    value: reservation_affinity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourcePolicies".into(),
                    value: resource_policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduling".into(),
                    value: scheduling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scratchDisks".into(),
                    value: scratch_disks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: service_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shieldedInstanceConfig".into(),
                    value: shielded_instance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceInstanceTemplate".into(),
                    value: source_instance_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceFromTemplateResult {
            advanced_machine_features: o.get_field("advancedMachineFeatures"),
            allow_stopping_for_update: o.get_field("allowStoppingForUpdate"),
            attached_disks: o.get_field("attachedDisks"),
            boot_disk: o.get_field("bootDisk"),
            can_ip_forward: o.get_field("canIpForward"),
            confidential_instance_config: o.get_field("confidentialInstanceConfig"),
            cpu_platform: o.get_field("cpuPlatform"),
            creation_timestamp: o.get_field("creationTimestamp"),
            current_status: o.get_field("currentStatus"),
            deletion_protection: o.get_field("deletionProtection"),
            description: o.get_field("description"),
            desired_status: o.get_field("desiredStatus"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_display: o.get_field("enableDisplay"),
            guest_accelerators: o.get_field("guestAccelerators"),
            hostname: o.get_field("hostname"),
            instance_id: o.get_field("instanceId"),
            key_revocation_action_type: o.get_field("keyRevocationActionType"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            machine_type: o.get_field("machineType"),
            metadata: o.get_field("metadata"),
            metadata_fingerprint: o.get_field("metadataFingerprint"),
            metadata_startup_script: o.get_field("metadataStartupScript"),
            min_cpu_platform: o.get_field("minCpuPlatform"),
            name: o.get_field("name"),
            network_interfaces: o.get_field("networkInterfaces"),
            network_performance_config: o.get_field("networkPerformanceConfig"),
            params: o.get_field("params"),
            partner_metadata: o.get_field("partnerMetadata"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reservation_affinity: o.get_field("reservationAffinity"),
            resource_policies: o.get_field("resourcePolicies"),
            scheduling: o.get_field("scheduling"),
            scratch_disks: o.get_field("scratchDisks"),
            self_link: o.get_field("selfLink"),
            service_account: o.get_field("serviceAccount"),
            shielded_instance_config: o.get_field("shieldedInstanceConfig"),
            source_instance_template: o.get_field("sourceInstanceTemplate"),
            tags: o.get_field("tags"),
            tags_fingerprint: o.get_field("tagsFingerprint"),
            zone: o.get_field("zone"),
        }
    }
}
