#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_group_manager {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceGroupManagerArgs {
        /// The name of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self link of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance group. If referencing the instance group by name and `zone` is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceGroupManagerResult {
        pub all_instances_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerAllInstancesConfig,
            >,
        >,
        pub auto_healing_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerAutoHealingPolicy,
            >,
        >,
        pub base_instance_name: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_group: pulumi_gestalt_rust::Output<String>,
        pub instance_group_manager_id: pulumi_gestalt_rust::Output<i32>,
        pub instance_lifecycle_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        pub list_managed_instances_results: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub named_ports: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerNamedPort>,
        >,
        pub operation: pulumi_gestalt_rust::Output<String>,
        pub params: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerParam>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<Option<String>>,
        pub standby_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStandbyPolicy,
            >,
        >,
        pub stateful_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerStatefulDisk>,
        >,
        pub stateful_external_ips: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStatefulExternalIp,
            >,
        >,
        pub stateful_internal_ips: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStatefulInternalIp,
            >,
        >,
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerStatus>,
        >,
        pub target_pools: pulumi_gestalt_rust::Output<Vec<String>>,
        pub target_size: pulumi_gestalt_rust::Output<i32>,
        pub target_stopped_size: pulumi_gestalt_rust::Output<i32>,
        pub target_suspended_size: pulumi_gestalt_rust::Output<i32>,
        pub update_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerUpdatePolicy>,
        >,
        pub versions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerVersion>,
        >,
        pub wait_for_instances: pulumi_gestalt_rust::Output<bool>,
        pub wait_for_instances_status: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceGroupManagerArgs,
    ) -> GetInstanceGroupManagerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let self_link_binding = args.self_link.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getInstanceGroupManager:getInstanceGroupManager".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceGroupManagerResult {
            all_instances_configs: o.get_field("allInstancesConfigs"),
            auto_healing_policies: o.get_field("autoHealingPolicies"),
            base_instance_name: o.get_field("baseInstanceName"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            id: o.get_field("id"),
            instance_group: o.get_field("instanceGroup"),
            instance_group_manager_id: o.get_field("instanceGroupManagerId"),
            instance_lifecycle_policies: o.get_field("instanceLifecyclePolicies"),
            list_managed_instances_results: o.get_field("listManagedInstancesResults"),
            name: o.get_field("name"),
            named_ports: o.get_field("namedPorts"),
            operation: o.get_field("operation"),
            params: o.get_field("params"),
            project: o.get_field("project"),
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
            zone: o.get_field("zone"),
        }
    }
}
