pub mod get_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of the instance. One of `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If `self_link` is provided, this value is ignored.  If neither `self_link`
        /// nor `project` are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The self link of the instance. One of `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone of the instance. If `self_link` is provided, this
        /// value is ignored.  If neither `self_link` nor `zone` are provided, the
        /// provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub advanced_machine_features: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceAdvancedMachineFeature>,
        >,
        pub allow_stopping_for_update: pulumi_wasm_rust::Output<bool>,
        /// List of disks attached to the instance. Structure is documented below.
        pub attached_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceAttachedDisk>,
        >,
        /// The boot disk for the instance. Structure is documented below.
        pub boot_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceBootDisk>,
        >,
        /// Whether sending and receiving of packets with non-matching source or destination IPs is allowed.
        pub can_ip_forward: pulumi_wasm_rust::Output<bool>,
        pub confidential_instance_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceConfidentialInstanceConfig,
            >,
        >,
        /// The CPU platform used by this instance.
        pub cpu_platform: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// The current status of the instance. This could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. For more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle).
        pub current_status: pulumi_wasm_rust::Output<String>,
        /// Whether deletion protection is enabled on this instance.
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        /// A brief description of the resource.
        pub description: pulumi_wasm_rust::Output<String>,
        pub desired_status: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the instance has virtual displays enabled.
        pub enable_display: pulumi_wasm_rust::Output<bool>,
        /// List of the type and count of accelerator cards attached to the instance. Structure is documented below.
        pub guest_accelerators: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGuestAccelerator>,
        >,
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The server-assigned unique identifier of this instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked.
        pub key_revocation_action_type: pulumi_wasm_rust::Output<String>,
        /// The unique fingerprint of the labels.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// A set of key/value label pairs assigned to the disk.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The machine type to create.
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Metadata key/value pairs made available within the instance.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_wasm_rust::Output<String>,
        pub metadata_startup_script: pulumi_wasm_rust::Output<String>,
        /// The minimum CPU platform specified for the VM instance. Set to "AUTOMATIC" to remove a previously-set value.
        pub min_cpu_platform: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks attached to the instance. Structure is documented below.
        pub network_interfaces: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceNetworkInterface>,
        >,
        /// The network performance configuration setting for the instance, if set. Structure is documented below.
        pub network_performance_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceNetworkPerformanceConfig>,
        >,
        pub params: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceParam>,
        >,
        pub partner_metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reservation_affinities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceReservationAffinity>,
        >,
        /// A list of self_links to resource policies attached to the selected `boot_disk`
        pub resource_policies: pulumi_wasm_rust::Output<Vec<String>>,
        /// The scheduling strategy being used by the instance. Structure is documented below
        pub schedulings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceScheduling>,
        >,
        /// The scratch disks attached to the instance. Structure is documented below.
        pub scratch_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceScratchDisk>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        /// The service account to attach to the instance. Structure is documented below.
        pub service_accounts: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceServiceAccount>,
        >,
        /// The shielded vm config being used by the instance. Structure is documented below.
        pub shielded_instance_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceShieldedInstanceConfig>,
        >,
        /// The list of tags attached to the instance.
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceArgs) -> GetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let self_link_binding = args.self_link.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getInstance:getInstance".into(),
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
                    name: "bootDisks".into(),
                },
                register_interface::ResultField {
                    name: "canIpForward".into(),
                },
                register_interface::ResultField {
                    name: "confidentialInstanceConfigs".into(),
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
                    name: "id".into(),
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
                    name: "networkPerformanceConfigs".into(),
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
                    name: "reservationAffinities".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicies".into(),
                },
                register_interface::ResultField {
                    name: "schedulings".into(),
                },
                register_interface::ResultField {
                    name: "scratchDisks".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccounts".into(),
                },
                register_interface::ResultField {
                    name: "shieldedInstanceConfigs".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceResult {
            advanced_machine_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedMachineFeatures").unwrap(),
            ),
            allow_stopping_for_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowStoppingForUpdate").unwrap(),
            ),
            attached_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedDisks").unwrap(),
            ),
            boot_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootDisks").unwrap(),
            ),
            can_ip_forward: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("canIpForward").unwrap(),
            ),
            confidential_instance_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialInstanceConfigs").unwrap(),
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            network_performance_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkPerformanceConfigs").unwrap(),
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
            reservation_affinities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationAffinities").unwrap(),
            ),
            resource_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicies").unwrap(),
            ),
            schedulings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulings").unwrap(),
            ),
            scratch_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scratchDisks").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccounts").unwrap(),
            ),
            shielded_instance_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shieldedInstanceConfigs").unwrap(),
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
