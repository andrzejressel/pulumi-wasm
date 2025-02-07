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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGroupArgs,
    ) -> GetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alternate_identifier_binding = args
            .alternate_identifier
            .get_output(context)
            .get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let group_id_binding = args.group_id.get_output(context).get_inner();
        let identity_store_id_binding = args
            .identity_store_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getGroup:getGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alternateIdentifier".into(),
                    value: &alternate_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupResult {
            alternate_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("alternateIdentifier"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            external_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalIds"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityStoreId"),
            ),
        }
    }
}
