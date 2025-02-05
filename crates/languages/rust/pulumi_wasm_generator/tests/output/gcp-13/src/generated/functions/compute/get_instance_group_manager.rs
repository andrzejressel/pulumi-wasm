pub mod get_instance_group_manager {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceGroupManagerArgs {
        /// The name of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The self link of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance group. If referencing the instance group by name and `zone` is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceGroupManagerResult {
        pub all_instances_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerAllInstancesConfig,
            >,
        >,
        pub auto_healing_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerAutoHealingPolicy,
            >,
        >,
        pub base_instance_name: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_group: pulumi_wasm_rust::Output<String>,
        pub instance_group_manager_id: pulumi_wasm_rust::Output<i32>,
        pub instance_lifecycle_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        pub list_managed_instances_results: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub named_ports: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerNamedPort>,
        >,
        pub operation: pulumi_wasm_rust::Output<String>,
        pub params: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerParam>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        pub standby_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStandbyPolicy,
            >,
        >,
        pub stateful_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerStatefulDisk>,
        >,
        pub stateful_external_ips: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStatefulExternalIp,
            >,
        >,
        pub stateful_internal_ips: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGroupManagerStatefulInternalIp,
            >,
        >,
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerStatus>,
        >,
        pub target_pools: pulumi_wasm_rust::Output<Vec<String>>,
        pub target_size: pulumi_wasm_rust::Output<i32>,
        pub target_stopped_size: pulumi_wasm_rust::Output<i32>,
        pub target_suspended_size: pulumi_wasm_rust::Output<i32>,
        pub update_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerUpdatePolicy>,
        >,
        pub versions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetInstanceGroupManagerVersion>,
        >,
        pub wait_for_instances: pulumi_wasm_rust::Output<bool>,
        pub wait_for_instances_status: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceGroupManagerArgs,
    ) -> GetInstanceGroupManagerResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let self_link_binding = args.self_link.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getInstanceGroupManager:getInstanceGroupManager".into(),
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
        GetInstanceGroupManagerResult {
            all_instances_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allInstancesConfigs"),
            ),
            auto_healing_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoHealingPolicies"),
            ),
            base_instance_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseInstanceName"),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceGroup"),
            ),
            instance_group_manager_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceGroupManagerId"),
            ),
            instance_lifecycle_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceLifecyclePolicies"),
            ),
            list_managed_instances_results: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listManagedInstancesResults"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            named_ports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namedPorts"),
            ),
            operation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operation"),
            ),
            params: pulumi_wasm_rust::__private::into_domain(o.extract_field("params")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            standby_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("standbyPolicies"),
            ),
            stateful_disks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statefulDisks"),
            ),
            stateful_external_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statefulExternalIps"),
            ),
            stateful_internal_ips: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statefulInternalIps"),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            target_pools: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetPools"),
            ),
            target_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetSize"),
            ),
            target_stopped_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetStoppedSize"),
            ),
            target_suspended_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetSuspendedSize"),
            ),
            update_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updatePolicies"),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versions"),
            ),
            wait_for_instances: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForInstances"),
            ),
            wait_for_instances_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("waitForInstancesStatus"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
