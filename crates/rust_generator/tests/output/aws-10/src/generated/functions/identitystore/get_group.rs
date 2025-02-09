#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupArgs {
        /// A unique identifier for the group that is not the primary identifier. Conflicts with `group_id` and `filter`. Detailed below.
        #[builder(into, default)]
        pub alternate_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::super::types::identitystore::GetGroupAlternateIdentifier,
            >,
        >,
        /// Configuration block for filtering by a unique attribute of the group. Detailed below.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::identitystore::GetGroupFilter>,
        >,
        /// The identifier for a group in the Identity Store.
        ///
        /// > Exactly one of the above arguments must be provided. Passing both `filter` and `group_id` is allowed for backwards compatibility.
        #[builder(into, default)]
        pub group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identity Store ID associated with the Single Sign-On Instance.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupResult {
        pub alternate_identifier: pulumi_gestalt_rust::Output<
            Option<
                super::super::super::types::identitystore::GetGroupAlternateIdentifier,
            >,
        >,
        /// Description of the specified group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Group's display name value.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// List of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetGroupExternalId>,
        >,
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::identitystore::GetGroupFilter>,
        >,
        pub group_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alternate_identifier_binding = args.alternate_identifier.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let group_id_binding = args.group_id.get_output(context);
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:identitystore/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alternateIdentifier".into(),
                    value: alternate_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupId".into(),
                    value: group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: identity_store_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGroupResult {
            alternate_identifier: o.get_field("alternateIdentifier"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            external_ids: o.get_field("externalIds"),
            filter: o.get_field("filter"),
            group_id: o.get_field("groupId"),
            id: o.get_field("id"),
            identity_store_id: o.get_field("identityStoreId"),
        }
    }
}
