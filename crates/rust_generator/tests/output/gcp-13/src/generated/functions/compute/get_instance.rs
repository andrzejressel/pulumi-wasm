#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of the instance. One of `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If `self_link` is provided, this value is ignored.  If neither `self_link`
        /// nor `project` are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self link of the instance. One of `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance. If `self_link` is provided, this
        /// value is ignored.  If neither `self_link` nor `zone` are provided, the
        /// provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub advanced_machine_features: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceAdvancedMachineFeature>,
        >,
        pub allow_stopping_for_update: pulumi_gestalt_rust::Output<bool>,
        /// List of disks attached to the instance. Structure is documented below.
        pub attached_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceAttachedDisk>,
        >,
        /// The boot disk for the instance. Structure is documented below.
        pub boot_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceBootDisk>,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        pub can_ip_forward: pulumi_gestalt_rust::Output<bool>,
        pub confidential_instance_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceConfidentialInstanceConfig,
            >,
        >,
        /// The CPU platform used by this instance.
        pub cpu_platform: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The current status of the instance. This could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle).
        pub current_status: pulumi_gestalt_rust::Output<String>,
        /// Whether deletion protection is enabled on this instance.
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        /// A brief description of the resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub desired_status: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the instance has virtual displays enabled.
        pub enable_display: pulumi_gestalt_rust::Output<bool>,
        /// List of the type and count of accelerator cards attached to the instance. Structure is documented below.
        pub guest_accelerators: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGuestAccelerator>,
        >,
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The server-assigned unique identifier of this instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked.
        pub key_revocation_action_type: pulumi_gestalt_rust::Output<String>,
        /// The unique fingerprint of the labels.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// A set of key/value label pairs assigned to the disk.
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
        pub metadata_startup_script: pulumi_gestalt_rust::Output<String>,
        /// The minimum CPU platform specified for the VM instance. Set to "AUTOMATIC" to remove a previously-set value.
        pub min_cpu_platform: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The networks attached to the instance. Structure is documented below.
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceNetworkInterface>,
        >,
        /// The network performance configuration setting for the instance, if set. Structure is documented below.
        pub network_performance_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceNetworkPerformanceConfig>,
        >,
        pub params: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceParam>,
        >,
        pub partner_metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reservation_affinities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceReservationAffinity>,
        >,
        /// A list of self_links to resource policies attached to the selected `boot_disk`
        pub resource_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The scheduling strategy being used by the instance. Structure is documented below
        pub schedulings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceScheduling>,
        >,
        /// The scratch disks attached to the instance. Structure is documented below.
        pub scratch_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceScratchDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service account to attach to the instance. Structure is documented below.
        pub service_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceServiceAccount>,
        >,
        /// The shielded vm config being used by the instance. Structure is documented below.
        pub shielded_instance_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceShieldedInstanceConfig>,
        >,
        /// The list of tags attached to the instance.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: self_link_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceResult {
            advanced_machine_features: o.get_field("advancedMachineFeatures"),
            allow_stopping_for_update: o.get_field("allowStoppingForUpdate"),
            attached_disks: o.get_field("attachedDisks"),
            boot_disks: o.get_field("bootDisks"),
            can_ip_forward: o.get_field("canIpForward"),
            confidential_instance_configs: o.get_field("confidentialInstanceConfigs"),
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
            id: o.get_field("id"),
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
            network_performance_configs: o.get_field("networkPerformanceConfigs"),
            params: o.get_field("params"),
            partner_metadata: o.get_field("partnerMetadata"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reservation_affinities: o.get_field("reservationAffinities"),
            resource_policies: o.get_field("resourcePolicies"),
            schedulings: o.get_field("schedulings"),
            scratch_disks: o.get_field("scratchDisks"),
            self_link: o.get_field("selfLink"),
            service_accounts: o.get_field("serviceAccounts"),
            shielded_instance_configs: o.get_field("shieldedInstanceConfigs"),
            tags: o.get_field("tags"),
            tags_fingerprint: o.get_field("tagsFingerprint"),
            zone: o.get_field("zone"),
        }
    }
}
