/// Resource for managing an AWS IdentityStore Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let this = group::create(
///         "this",
///         GroupArgs::builder()
///             .description("Example description")
///             .display_name("Example group")
///             .identity_store_id("${example.identityStoreIds[0]}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Identity Store Group using the combination `identity_store_id/group_id`. For example:
///
/// ```sh
/// $ pulumi import aws:identitystore/group:Group example d-9c6705e95c/b8a1c340-8031-7071-a2fb-7dc540320c30
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// A string containing the description of the group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A string containing the name of the group. This value is commonly displayed when the group is referenced.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The globally unique identifier for the identity store.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// A string containing the description of the group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A string containing the name of the group. This value is commonly displayed when the group is referenced.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// A list of external IDs that contains the identifiers issued to this resource by an external identity provider. See External IDs below.
        pub external_ids: pulumi_gestalt_rust::Output<
            Vec<super::super::types::identitystore::GroupExternalId>,
        >,
        /// The identifier of the newly created group in the identity store.
        pub group_id: pulumi_gestalt_rust::Output<String>,
        /// The globally unique identifier for the identity store.
        ///
        /// The following arguments are optional:
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:identitystore/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: identity_store_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            external_ids: o.get_field("externalIds"),
            group_id: o.get_field("groupId"),
            identity_store_id: o.get_field("identityStoreId"),
        }
    }
}
