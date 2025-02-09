#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_region_instance_group_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupManagerArgs {
        /// The name of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        ///
        /// * `Region` - (Optional) The region where the managed instance group resides. If not provided, the provider region is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self link of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupManagerResult {
        pub all_instances_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerAllInstancesConfig,
            >,
        >,
        pub auto_healing_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerAutoHealingPolicy,
            >,
        >,
        pub base_instance_name: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub distribution_policy_target_shape: pulumi_gestalt_rust::Output<String>,
        pub distribution_policy_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_flexibility_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerInstanceFlexibilityPolicy,
            >,
        >,
        pub instance_group: pulumi_gestalt_rust::Output<String>,
        pub instance_group_manager_id: pulumi_gestalt_rust::Output<i32>,
        pub instance_lifecycle_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        pub list_managed_instances_results: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub named_ports: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerNamedPort,
            >,
        >,
        pub params: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionInstanceGroupManagerParam>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        pub standby_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStandbyPolicy,
            >,
        >,
        pub stateful_disks: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulDisk,
            >,
        >,
        pub stateful_external_ips: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulExternalIp,
            >,
        >,
        pub stateful_internal_ips: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulInternalIp,
            >,
        >,
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionInstanceGroupManagerStatus>,
        >,
        pub target_pools: pulumi_gestalt_rust::Output<Vec<String>>,
        pub target_size: pulumi_gestalt_rust::Output<i32>,
        pub target_stopped_size: pulumi_gestalt_rust::Output<i32>,
        pub target_suspended_size: pulumi_gestalt_rust::Output<i32>,
        pub update_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerUpdatePolicy,
            >,
        >,
        pub versions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerVersion,
            >,
        >,
        pub wait_for_instances: pulumi_gestalt_rust::Output<bool>,
        pub wait_for_instances_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionInstanceGroupManagerArgs,
    ) -> GetRegionInstanceGroupManagerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getRegionInstanceGroupManager:getRegionInstanceGroupManager"
                .into(),
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
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: self_link_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionInstanceGroupManagerResult {
            all_instances_configs: o.get_field("allInstancesConfigs"),
            auto_healing_policies: o.get_field("autoHealingPolicies"),
            base_instance_name: o.get_field("baseInstanceName"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            distribution_policy_target_shape: o
                .get_field("distributionPolicyTargetShape"),
            distribution_policy_zones: o.get_field("distributionPolicyZones"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            instance_flexibility_policies: o.get_field("instanceFlexibilityPolicies"),
            instance_group: o.get_field("instanceGroup"),
            instance_group_manager_id: o.get_field("instanceGroupManagerId"),
            instance_lifecycle_policies: o.get_field("instanceLifecyclePolicies"),
            list_managed_instances_results: o.get_field("listManagedInstancesResults"),
            name: o.get_field("name"),
            named_ports: o.get_field("namedPorts"),
            params: o.get_field("params"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            standby_policies: o.get_field("standbyPolicies"),
            stateful_disks: o.get_field("statefulDisks"),
            stateful_external_ips: o.get_field("statefulExternalIps"),
            stateful_internal_ips: o.get_field("statefulInternalIps"),
            statuses: o.get_field("statuses"),
            target_pools: o.get_field("targetPools"),
            target_size: o.get_field("targetSize"),
            target_stopped_size: o.get_field("targetStoppedSize"),
            target_suspended_size: o.get_field("targetSuspendedSize"),
            update_policies: o.get_field("updatePolicies"),
            versions: o.get_field("versions"),
            wait_for_instances: o.get_field("waitForInstances"),
            wait_for_instances_status: o.get_field("waitForInstancesStatus"),
        }
    }
}
