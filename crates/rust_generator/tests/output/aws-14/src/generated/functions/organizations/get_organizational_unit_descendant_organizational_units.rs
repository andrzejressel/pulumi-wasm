#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_organizational_unit_descendant_organizational_units {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitDescendantOrganizationalUnitsArgs {
        /// Parent ID of the organizational unit.
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitDescendantOrganizationalUnitsResult {
        /// List of child organizational units, which have the following attributes:
        pub childrens: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationalUnitDescendantOrganizationalUnitsChildren,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOrganizationalUnitDescendantOrganizationalUnitsArgs,
    ) -> GetOrganizationalUnitDescendantOrganizationalUnitsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_id_binding = args.parent_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getOrganizationalUnitDescendantOrganizationalUnits:getOrganizationalUnitDescendantOrganizationalUnits"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOrganizationalUnitDescendantOrganizationalUnitsResult {
            childrens: o.get_field("childrens"),
            id: o.get_field("id"),
            parent_id: o.get_field("parentId"),
        }
    }
}
