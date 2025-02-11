#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTemplateArgs {
        /// A filter to retrieve the instance templates.
        /// See [API filter parameter documentation](https://cloud.google.com/compute/docs/reference/rest/v1/instanceTemplates/list#body.QUERY_PARAMETERS.filter) for reference.
        /// If multiple instance templates match, either adjust the filter or specify `most_recent`.
        /// One of `name`, `filter` or `self_link_unique` must be provided.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `filter` is provided, ensures the most recent template is returned when multiple instance templates match. One of `name`, `filter` or `self_link_unique` must be provided.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the instance template. One of `name`, `filter` or `self_link_unique` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self_link_unique URI of the instance template. One of `name`, `filter` or `self_link_unique` must be provided.
        #[builder(into, default)]
        pub self_link_unique: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTemplateResult {
        pub advanced_machine_features: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceTemplateAdvancedMachineFeature,
            >,
        >,
        /// Whether to allow sending and receiving of
        /// packets with non-matching source or destination IPs. This defaults to false.
        pub can_ip_forward: pulumi_gestalt_rust::Output<bool>,
        /// Enable [Confidential Mode](https://cloud.google.com/compute/confidential-vm/docs/about-cvm) on this VM. Structure is documented below
        pub confidential_instance_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceTemplateConfidentialInstanceConfig,
            >,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// A brief description of this resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Disks to attach to instances created from this template.
        /// This can be specified multiple times for multiple disks. Structure is
        /// documented below.
        pub disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceTemplateDisk>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable [Virtual Displays](https://cloud.google.com/compute/docs/instances/enable-instance-virtual-display#verify_display_driver) on this instance.
        /// **Note**: `allow_stopping_for_update` must be set to true in order to update this field.
        pub enable_display: pulumi_gestalt_rust::Output<bool>,
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of the type and count of accelerator cards attached to the instance. Structure documented below.
        pub guest_accelerators: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceTemplateGuestAccelerator>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A brief description to use for instances
        /// created from this template.
        pub instance_description: pulumi_gestalt_rust::Output<String>,
        /// Action to be taken when a customer's encryption key is revoked.
        pub key_revocation_action_type: pulumi_gestalt_rust::Output<String>,
        /// (Optional) A set of ket/value label pairs to assign to disk created from
        /// this template
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The machine type to create.
        pub machine_type: pulumi_gestalt_rust::Output<String>,
        /// Metadata key/value pairs to make available from
        /// within instances created from this template.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique fingerprint of the metadata.
        pub metadata_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// An alternative to using the
        /// startup-script metadata key, mostly to match the compute_instance resource.
        /// This replaces the startup-script metadata key on the created instance and
        /// thus the two mechanisms are not allowed to be used simultaneously.
        pub metadata_startup_script: pulumi_gestalt_rust::Output<String>,
        /// Specifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as
        /// `Intel Haswell` or `Intel Skylake`. See the complete list [here](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform).
        pub min_cpu_platform: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the instance template. If you leave
        /// this blank, the provider will auto-generate a unique name.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}.  s
        pub network_interfaces: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceTemplateNetworkInterface>,
        >,
        /// The network performance configuration setting
        /// for the instance, if set. Structure is documented below.
        pub network_performance_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceTemplateNetworkPerformanceConfig,
            >,
        >,
        pub partner_metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An instance template is a global resource that is not
        /// bound to a zone or a region. However, you can still specify some regional
        /// resources in an instance template, which restricts the template to the
        /// region where that resource resides. For example, a custom `subnetwork`
        /// resource is tied to a specific region. Defaults to the region of the
        /// Provider if no value is given.
        pub region: pulumi_gestalt_rust::Output<String>,
        pub reservation_affinities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceTemplateReservationAffinity,
            >,
        >,
        pub resource_manager_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// (Optional) -- A list of short names of resource policies to attach to this disk for automatic snapshot creations. Currently a max of 1 resource policy is supported.
        pub resource_policies: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The scheduling strategy to use. More details about
        /// this configuration option are detailed below.
        pub schedulings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceTemplateScheduling>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A special URI of the created resource that uniquely identifies this instance template with the following format: `projects/{{project}}/global/instanceTemplates/{{name}}?uniqueId={{uniqueId}}`
        /// Referencing an instance template via this attribute prevents Time of Check to Time of Use attacks when the instance template resides in a shared/untrusted environment.
        pub self_link_unique: pulumi_gestalt_rust::Output<Option<String>>,
        /// Service account to attach to the instance. Structure is documented below.
        pub service_accounts: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceTemplateServiceAccount>,
        >,
        /// Enable [Shielded VM](https://cloud.google.com/security/shielded-cloud/shielded-vm) on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Structure is documented below.
        /// **Note**: `shielded_instance_config` can only be used with boot images with shielded vm support. See the complete list [here](https://cloud.google.com/compute/docs/images#shielded-images).
        pub shielded_instance_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceTemplateShieldedInstanceConfig,
            >,
        >,
        /// Tags to attach to the instance.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique fingerprint of the tags.
        pub tags_fingerprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceTemplateArgs,
    ) -> GetInstanceTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let self_link_unique_binding = args.self_link_unique.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getInstanceTemplate:getInstanceTemplate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLinkUnique".into(),
                    value: &self_link_unique_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceTemplateResult {
            advanced_machine_features: o.get_field("advancedMachineFeatures"),
            can_ip_forward: o.get_field("canIpForward"),
            confidential_instance_configs: o.get_field("confidentialInstanceConfigs"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            disks: o.get_field("disks"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_display: o.get_field("enableDisplay"),
            filter: o.get_field("filter"),
            guest_accelerators: o.get_field("guestAccelerators"),
            id: o.get_field("id"),
            instance_description: o.get_field("instanceDescription"),
            key_revocation_action_type: o.get_field("keyRevocationActionType"),
            labels: o.get_field("labels"),
            machine_type: o.get_field("machineType"),
            metadata: o.get_field("metadata"),
            metadata_fingerprint: o.get_field("metadataFingerprint"),
            metadata_startup_script: o.get_field("metadataStartupScript"),
            min_cpu_platform: o.get_field("minCpuPlatform"),
            most_recent: o.get_field("mostRecent"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            network_interfaces: o.get_field("networkInterfaces"),
            network_performance_configs: o.get_field("networkPerformanceConfigs"),
            partner_metadata: o.get_field("partnerMetadata"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            reservation_affinities: o.get_field("reservationAffinities"),
            resource_manager_tags: o.get_field("resourceManagerTags"),
            resource_policies: o.get_field("resourcePolicies"),
            schedulings: o.get_field("schedulings"),
            self_link: o.get_field("selfLink"),
            self_link_unique: o.get_field("selfLinkUnique"),
            service_accounts: o.get_field("serviceAccounts"),
            shielded_instance_configs: o.get_field("shieldedInstanceConfigs"),
            tags: o.get_field("tags"),
            tags_fingerprint: o.get_field("tagsFingerprint"),
        }
    }
}
