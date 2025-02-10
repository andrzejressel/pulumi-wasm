#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user_hierarchy_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserHierarchyGroupArgs {
        /// Returns information on a specific hierarchy group by hierarchy group id
        #[builder(into, default)]
        pub hierarchy_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific hierarchy group by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the hierarchy group.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetUserHierarchyGroupResult {
        /// ARN of the hierarchy group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub hierarchy_group_id: pulumi_gestalt_rust::Output<String>,
        /// Block that contains information about the levels in the hierarchy group. The `hierarchy_path` block is documented below.
        pub hierarchy_paths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetUserHierarchyGroupHierarchyPath>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the level in the hierarchy group.
        pub level_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the hierarchy group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the hierarchy group.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserHierarchyGroupArgs,
    ) -> GetUserHierarchyGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hierarchy_group_id_binding = args.hierarchy_group_id.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getUserHierarchyGroup:getUserHierarchyGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hierarchyGroupId".into(),
                    value: hierarchy_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserHierarchyGroupResult {
            arn: o.get_field("arn"),
            hierarchy_group_id: o.get_field("hierarchyGroupId"),
            hierarchy_paths: o.get_field("hierarchyPaths"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            level_id: o.get_field("levelId"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
