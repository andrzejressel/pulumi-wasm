pub mod get_group_memberships {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupMembershipsArgs {
        /// The parent Group resource under which to lookup the Membership names. Must be of the form groups/{group_id}.
        #[builder(into)]
        pub group: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupMembershipsArgs,
    ) -> GetGroupMembershipsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroupMemberships:getGroupMemberships".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupMembershipsResult {
            group: pulumi_wasm_rust::__private::into_domain(o.extract_field("group")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            memberships: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("memberships"),
            ),
        }
    }
}
