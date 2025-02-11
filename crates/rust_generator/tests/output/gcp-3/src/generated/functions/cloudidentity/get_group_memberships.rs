#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group_memberships {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupMembershipsArgs {
        /// The parent Group resource under which to lookup the Membership names. Must be of the form groups/{group_id}.
        #[builder(into)]
        pub group: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupMembershipsResult {
        pub group: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The list of memberships under the given group. Structure is documented below.
        pub memberships: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudidentity::GetGroupMembershipsMembership>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupMembershipsArgs,
    ) -> GetGroupMembershipsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_binding = args.group.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudidentity/getGroupMemberships:getGroupMemberships".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "group".into(),
                    value: &group_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupMembershipsResult {
            group: o.get_field("group"),
            id: o.get_field("id"),
            memberships: o.get_field("memberships"),
        }
    }
}
