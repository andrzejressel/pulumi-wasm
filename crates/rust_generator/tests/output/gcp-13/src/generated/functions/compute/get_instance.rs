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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let self_link_binding_1 = args.self_link.get_output(context);
        let self_link_binding = self_link_binding_1.get_inner();
        let zone_binding_1 = args.zone.get_output(context);
        let zone_binding = zone_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceResult {
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
            confidential_instance_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialInstanceConfigs"),
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
            network_performance_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkPerformanceConfigs"),
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
            reservation_affinities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservationAffinities"),
            ),
            resource_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourcePolicies"),
            ),
            schedulings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schedulings"),
            ),
            scratch_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scratchDisks"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccounts"),
            ),
            shielded_instance_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shieldedInstanceConfigs"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsFingerprint"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
