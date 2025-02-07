pub mod get_group_transitive_memberships {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupTransitiveMembershipsArgs {
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupTransitiveMembershipsResult {
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub memberships: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGroupTransitiveMembershipsArgs,
    ) -> GetGroupTransitiveMembershipsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupTransitiveMembershipsResult {
            group: pulumi_gestalt_rust::__private::into_domain(o.extract_field("group")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            memberships: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memberships"),
            ),
        }
    }
}
