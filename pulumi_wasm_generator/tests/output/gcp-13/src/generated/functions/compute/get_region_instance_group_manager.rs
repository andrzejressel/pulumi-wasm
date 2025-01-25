pub mod get_region_instance_group_manager {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupManagerArgs {
        /// The name of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        ///
        /// * `Region` - (Optional) The region where the managed instance group resides. If not provided, the provider region is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The self link of the instance group. Either `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupManagerResult {
        pub all_instances_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerAllInstancesConfig,
            >,
        >,
        pub auto_healing_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerAutoHealingPolicy,
            >,
        >,
        pub base_instance_name: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub distribution_policy_target_shape: pulumi_wasm_rust::Output<String>,
        pub distribution_policy_zones: pulumi_wasm_rust::Output<Vec<String>>,
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_flexibility_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerInstanceFlexibilityPolicy,
            >,
        >,
        pub instance_group: pulumi_wasm_rust::Output<String>,
        pub instance_group_manager_id: pulumi_wasm_rust::Output<i32>,
        pub instance_lifecycle_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerInstanceLifecyclePolicy,
            >,
        >,
        pub list_managed_instances_results: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub named_ports: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerNamedPort,
            >,
        >,
        pub params: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRegionInstanceGroupManagerParam>,
        >,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        pub standby_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStandbyPolicy,
            >,
        >,
        pub stateful_disks: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulDisk,
            >,
        >,
        pub stateful_external_ips: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulExternalIp,
            >,
        >,
        pub stateful_internal_ips: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerStatefulInternalIp,
            >,
        >,
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetRegionInstanceGroupManagerStatus>,
        >,
        pub target_pools: pulumi_wasm_rust::Output<Vec<String>>,
        pub target_size: pulumi_wasm_rust::Output<i32>,
        pub target_stopped_size: pulumi_wasm_rust::Output<i32>,
        pub target_suspended_size: pulumi_wasm_rust::Output<i32>,
        pub update_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerUpdatePolicy,
            >,
        >,
        pub versions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetRegionInstanceGroupManagerVersion,
            >,
        >,
        pub wait_for_instances: pulumi_wasm_rust::Output<bool>,
        pub wait_for_instances_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegionInstanceGroupManagerArgs,
    ) -> GetRegionInstanceGroupManagerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let self_link_binding = args.self_link.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getRegionInstanceGroupManager:getRegionInstanceGroupManager"
                .into(),
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allInstancesConfigs".into(),
                },
                register_interface::ResultField {
                    name: "autoHealingPolicies".into(),
                },
                register_interface::ResultField {
                    name: "baseInstanceName".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "distributionPolicyTargetShape".into(),
                },
                register_interface::ResultField {
                    name: "distributionPolicyZones".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceFlexibilityPolicies".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroup".into(),
                },
                register_interface::ResultField {
                    name: "instanceGroupManagerId".into(),
                },
                register_interface::ResultField {
                    name: "instanceLifecyclePolicies".into(),
                },
                register_interface::ResultField {
                    name: "listManagedInstancesResults".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namedPorts".into(),
                },
                register_interface::ResultField {
                    name: "params".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "standbyPolicies".into(),
                },
                register_interface::ResultField {
                    name: "statefulDisks".into(),
                },
                register_interface::ResultField {
                    name: "statefulExternalIps".into(),
                },
                register_interface::ResultField {
                    name: "statefulInternalIps".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "targetPools".into(),
                },
                register_interface::ResultField {
                    name: "targetSize".into(),
                },
                register_interface::ResultField {
                    name: "targetStoppedSize".into(),
                },
                register_interface::ResultField {
                    name: "targetSuspendedSize".into(),
                },
                register_interface::ResultField {
                    name: "updatePolicies".into(),
                },
                register_interface::ResultField {
                    name: "versions".into(),
                },
                register_interface::ResultField {
                    name: "waitForInstances".into(),
                },
                register_interface::ResultField {
                    name: "waitForInstancesStatus".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegionInstanceGroupManagerResult {
            all_instances_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allInstancesConfigs").unwrap(),
            ),
            auto_healing_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoHealingPolicies").unwrap(),
            ),
            base_instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseInstanceName").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            distribution_policy_target_shape: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributionPolicyTargetShape").unwrap(),
            ),
            distribution_policy_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distributionPolicyZones").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_flexibility_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceFlexibilityPolicies").unwrap(),
            ),
            instance_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroup").unwrap(),
            ),
            instance_group_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceGroupManagerId").unwrap(),
            ),
            instance_lifecycle_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceLifecyclePolicies").unwrap(),
            ),
            list_managed_instances_results: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listManagedInstancesResults").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            named_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namedPorts").unwrap(),
            ),
            params: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("params").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            standby_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standbyPolicies").unwrap(),
            ),
            stateful_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulDisks").unwrap(),
            ),
            stateful_external_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulExternalIps").unwrap(),
            ),
            stateful_internal_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statefulInternalIps").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            target_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetPools").unwrap(),
            ),
            target_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetSize").unwrap(),
            ),
            target_stopped_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetStoppedSize").unwrap(),
            ),
            target_suspended_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetSuspendedSize").unwrap(),
            ),
            update_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatePolicies").unwrap(),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versions").unwrap(),
            ),
            wait_for_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForInstances").unwrap(),
            ),
            wait_for_instances_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForInstancesStatus").unwrap(),
            ),
        }
    }
}
