#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupTransitiveMembershipsArgs,
    ) -> GetGroupTransitiveMembershipsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudidentity/getGroupTransitiveMemberships:getGroupTransitiveMemberships"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: group_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupTransitiveMembershipsResult {
            group: o.get_field("group"),
            id: o.get_field("id"),
            memberships: o.get_field("memberships"),
        }
    }
}
