#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_groups {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupsArgs {
        /// The parent resource under which to list all Groups. Must be of the form identitysources/{identity_source_id} for external- identity-mapped groups or customers/{customer_id} for Google Groups.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupsResult {
        /// The list of groups under the provided customer or namespace. Structure is documented below.
        pub groups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudidentity::GetGroupsGroup>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGroupsArgs,
    ) -> GetGroupsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudidentity/getGroups:getGroups".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupsResult {
            groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groups"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
        }
    }
}
