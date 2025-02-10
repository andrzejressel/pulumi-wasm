/// Provides an Amazon Connect User Hierarchy Group resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// > **NOTE:** The User Hierarchy Structure must be created before creating a User Hierarchy Group.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:UserHierarchyGroup
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: example
///       tags:
///         Name: Example User Hierarchy Group
/// ```
///
/// ### With a parent group
///
/// ```yaml
/// resources:
///   parent:
///     type: aws:connect:UserHierarchyGroup
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: parent
///       tags:
///         Name: Example User Hierarchy Group Parent
///   child:
///     type: aws:connect:UserHierarchyGroup
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: child
///       parentGroupId: ${parent.hierarchyGroupId}
///       tags:
///         Name: Example User Hierarchy Group Child
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect User Hierarchy Groups using the `instance_id` and `hierarchy_group_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/userHierarchyGroup:UserHierarchyGroup example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_hierarchy_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserHierarchyGroupArgs {
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the user hierarchy group. Must not be more than 100 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier for the parent hierarchy group. The user hierarchy is created at level one if the parent group ID is null.
        #[builder(into, default)]
        pub parent_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the hierarchy group. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserHierarchyGroupResult {
        /// The Amazon Resource Name (ARN) of the hierarchy group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the hierarchy group.
        pub hierarchy_group_id: pulumi_gestalt_rust::Output<String>,
        /// A block that contains information about the levels in the hierarchy group. The `hierarchy_path` block is documented below.
        pub hierarchy_paths: pulumi_gestalt_rust::Output<
            Vec<super::super::types::connect::UserHierarchyGroupHierarchyPath>,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the level in the hierarchy group.
        pub level_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the user hierarchy group. Must not be more than 100 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the parent hierarchy group. The user hierarchy is created at level one if the parent group ID is null.
        pub parent_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tags to apply to the hierarchy group. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserHierarchyGroupArgs,
    ) -> UserHierarchyGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_group_id_binding = args.parent_group_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/userHierarchyGroup:UserHierarchyGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentGroupId".into(),
                    value: parent_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserHierarchyGroupResult {
            arn: o.get_field("arn"),
            hierarchy_group_id: o.get_field("hierarchyGroupId"),
            hierarchy_paths: o.get_field("hierarchyPaths"),
            instance_id: o.get_field("instanceId"),
            level_id: o.get_field("levelId"),
            name: o.get_field("name"),
            parent_group_id: o.get_field("parentGroupId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
