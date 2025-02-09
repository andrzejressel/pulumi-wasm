/// Provides an Amazon Connect User Hierarchy Structure resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_hierarchy_structure::create(
///         "example",
///         UserHierarchyStructureArgs::builder()
///             .hierarchy_structure(
///                 UserHierarchyStructureHierarchyStructure::builder()
///                     .levelOne(
///                         UserHierarchyStructureHierarchyStructureLevelOne::builder()
///                             .name("levelone")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .instance_id("aaaaaaaa-bbbb-cccc-dddd-111111111111")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Five Levels
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_hierarchy_structure::create(
///         "example",
///         UserHierarchyStructureArgs::builder()
///             .hierarchy_structure(
///                 UserHierarchyStructureHierarchyStructure::builder()
///                     .levelFive(
///                         UserHierarchyStructureHierarchyStructureLevelFive::builder()
///                             .name("levelfive")
///                             .build_struct(),
///                     )
///                     .levelFour(
///                         UserHierarchyStructureHierarchyStructureLevelFour::builder()
///                             .name("levelfour")
///                             .build_struct(),
///                     )
///                     .levelOne(
///                         UserHierarchyStructureHierarchyStructureLevelOne::builder()
///                             .name("levelone")
///                             .build_struct(),
///                     )
///                     .levelThree(
///                         UserHierarchyStructureHierarchyStructureLevelThree::builder()
///                             .name("levelthree")
///                             .build_struct(),
///                     )
///                     .levelTwo(
///                         UserHierarchyStructureHierarchyStructureLevelTwo::builder()
///                             .name("leveltwo")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .instance_id("aaaaaaaa-bbbb-cccc-dddd-111111111111")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect User Hierarchy Structures using the `instance_id`. For example:
///
/// ```sh
/// $ pulumi import aws:connect/userHierarchyStructure:UserHierarchyStructure example f1288a1f-6193-445a-b47e-af739b2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_hierarchy_structure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserHierarchyStructureArgs {
        /// A block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
        #[builder(into)]
        pub hierarchy_structure: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::connect::UserHierarchyStructureHierarchyStructure,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserHierarchyStructureResult {
        /// A block that defines the hierarchy structure's levels. The `hierarchy_structure` block is documented below.
        pub hierarchy_structure: pulumi_gestalt_rust::Output<
            super::super::types::connect::UserHierarchyStructureHierarchyStructure,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserHierarchyStructureArgs,
    ) -> UserHierarchyStructureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hierarchy_structure_binding = args.hierarchy_structure.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/userHierarchyStructure:UserHierarchyStructure".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hierarchyStructure".into(),
                    value: hierarchy_structure_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserHierarchyStructureResult {
            hierarchy_structure: o.get_field("hierarchyStructure"),
            instance_id: o.get_field("instanceId"),
        }
    }
}
