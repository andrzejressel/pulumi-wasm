pub mod get_resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcePolicyArgs {
        /// The name of the Resource Policy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Project from which to list the Resource Policy. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Region where the Resource Policy resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResourcePolicyResult {
        /// Description of this Resource Policy.
        pub description: pulumi_wasm_rust::Output<String>,
        pub disk_consistency_group_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyDiskConsistencyGroupPolicy,
            >,
        >,
        pub group_placement_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyGroupPlacementPolicy,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_schedule_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyInstanceSchedulePolicy,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of the resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub snapshot_schedule_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicySnapshotSchedulePolicy,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetResourcePolicyArgs) -> GetResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getResourcePolicy:getResourcePolicy".into(),
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskConsistencyGroupPolicies".into(),
                },
                register_interface::ResultField {
                    name: "groupPlacementPolicies".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceSchedulePolicies".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                    name: "snapshotSchedulePolicies".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResourcePolicyResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_consistency_group_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskConsistencyGroupPolicies").unwrap(),
            ),
            group_placement_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupPlacementPolicies").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_schedule_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceSchedulePolicies").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            snapshot_schedule_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotSchedulePolicies").unwrap(),
            ),
        }
    }
}
