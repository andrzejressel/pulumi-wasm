#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user_hierarchy_structure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserHierarchyStructureArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserHierarchyStructureResult {
        /// Block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
        pub hierarchy_structures: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::connect::GetUserHierarchyStructureHierarchyStructure,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetUserHierarchyStructureArgs,
    ) -> GetUserHierarchyStructureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getUserHierarchyStructure:getUserHierarchyStructure"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserHierarchyStructureResult {
            hierarchy_structures: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hierarchyStructures"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
        }
    }
}
