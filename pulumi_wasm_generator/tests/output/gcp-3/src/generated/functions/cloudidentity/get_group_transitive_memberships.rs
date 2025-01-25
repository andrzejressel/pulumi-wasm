pub mod get_group_transitive_memberships {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupTransitiveMembershipsArgs {
        #[builder(into)]
        pub group: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupTransitiveMembershipsResult {
        pub group: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub memberships: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudidentity::GetGroupTransitiveMembershipsMembership,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupTransitiveMembershipsArgs,
    ) -> GetGroupTransitiveMembershipsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroupTransitiveMemberships:getGroupTransitiveMemberships"
                .into(),
            version: super::super::super::get_version(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGroupTransitiveMembershipsResult {
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
