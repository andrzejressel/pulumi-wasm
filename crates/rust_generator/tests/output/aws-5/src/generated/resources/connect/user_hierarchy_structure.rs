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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserHierarchyStructureArgs,
    ) -> UserHierarchyStructureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let hierarchy_structure_binding_1 = args.hierarchy_structure.get_output(context);
        let hierarchy_structure_binding = hierarchy_structure_binding_1.get_inner();
        let instance_id_binding_1 = args.instance_id.get_output(context);
        let instance_id_binding = instance_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/userHierarchyStructure:UserHierarchyStructure".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hierarchyStructure".into(),
                    value: &hierarchy_structure_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserHierarchyStructureResult {
            hierarchy_structure: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hierarchyStructure"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
        }
    }
}
