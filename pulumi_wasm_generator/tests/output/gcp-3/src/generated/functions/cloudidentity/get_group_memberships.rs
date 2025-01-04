pub mod get_group_memberships {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupMembershipsArgs {
        /// The parent Group resource under which to lookup the Membership names. Must be of the form groups/{group_id}.
        #[builder(into)]
        pub group: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupMembershipsResult {
        pub group: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The list of memberships under the given group. Structure is documented below.
        pub memberships: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudidentity::GetGroupMembershipsMembership>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGroupMembershipsArgs) -> GetGroupMembershipsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroupMemberships:getGroupMemberships".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "group".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "memberships".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupMembershipsResult {
            group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("group").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            memberships: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberships").unwrap(),
            ),
        }
    }
}
