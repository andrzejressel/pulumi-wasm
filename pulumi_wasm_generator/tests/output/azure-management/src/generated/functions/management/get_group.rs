pub mod get_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// Specifies the display name of this Management Group.
        ///
        /// > **NOTE** Whilst multiple management groups may share the same display name, when filtering, the provider expects a single management group to be found with this name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name or UUID of this Management Group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        /// A list of Management Group IDs which directly or indirectly belong to this Management Group.
        pub all_management_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of Subscription IDs which are assigned to this Management Group or its children Management Groups.
        pub all_subscription_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of Management Group IDs which directly belong to this Management Group.
        pub management_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of any Parent Management Group.
        pub parent_management_group_id: pulumi_wasm_rust::Output<String>,
        /// A list of Subscription IDs which are directly assigned to this Management Group.
        pub subscription_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Management Group ID with the Tenant ID prefix.
        pub tenant_scoped_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGroupArgs) -> GetGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:management/getGroup:getGroup".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allManagementGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "allSubscriptionIds".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managementGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parentManagementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "subscriptionIds".into(),
                },
                register_interface::ResultField {
                    name: "tenantScopedId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupResult {
            all_management_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allManagementGroupIds").unwrap(),
            ),
            all_subscription_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allSubscriptionIds").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            management_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupIds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent_management_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentManagementGroupId").unwrap(),
            ),
            subscription_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subscriptionIds").unwrap(),
            ),
            tenant_scoped_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantScopedId").unwrap(),
            ),
        }
    }
}