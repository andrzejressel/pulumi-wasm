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
        context: &pulumi_gestalt_rust::Context,
        args: GetUserHierarchyStructureArgs,
    ) -> GetUserHierarchyStructureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getUserHierarchyStructure:getUserHierarchyStructure"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserHierarchyStructureResult {
            hierarchy_structures: o.get_field("hierarchyStructures"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
        }
    }
}
