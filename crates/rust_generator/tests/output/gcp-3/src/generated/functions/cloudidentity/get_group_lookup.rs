#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group_lookup {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupLookupArgs {
        /// The EntityKey of the Group to lookup. A unique identifier for an entity in the Cloud Identity Groups API.
        /// An entity can represent either a group with an optional namespace or a user without a namespace.
        /// The combination of id and namespace must be unique; however, the same id can be used with different namespaces. Structure is documented below.
        #[builder(into)]
        pub group_key: pulumi_gestalt_rust::InputOrOutput<
            super::super::super::types::cloudidentity::GetGroupLookupGroupKey,
        >,
    }
    #[allow(dead_code)]
    pub struct GetGroupLookupResult {
        pub group_key: pulumi_gestalt_rust::Output<
            super::super::super::types::cloudidentity::GetGroupLookupGroupKey,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Resource name of the Group in the format: groups/{group_id}, where `group_id` is the unique ID assigned to the Group.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupLookupArgs,
    ) -> GetGroupLookupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let group_key_binding = args.group_key.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudidentity/getGroupLookup:getGroupLookup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupKey".into(),
                    value: group_key_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupLookupResult {
            group_key: o.get_field("groupKey"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
