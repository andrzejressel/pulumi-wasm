/// The description of a dynamic collection of monitored resources. Each group
/// has a filter that is matched against monitored resources and their
/// associated metadata. If a group's filter matches an available monitored
/// resource, then that resource is a member of that group.
///
///
/// To get more information about Group, see:
///
/// * [API documentation](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.groups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/monitoring/groups/)
///
/// ## Example Usage
///
/// ### Monitoring Group Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = group::create(
///         "basic",
///         GroupArgs::builder()
///             .display_name("tf-test MonitoringGroup")
///             .filter("resource.metadata.region=\"europe-west2\"")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Monitoring Group Subgroup
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let parent = group::create(
///         "parent",
///         GroupArgs::builder()
///             .display_name("tf-test MonitoringParentGroup")
///             .filter("resource.metadata.region=\"europe-west2\"")
///             .build_struct(),
///     );
///     let subgroup = group::create(
///         "subgroup",
///         GroupArgs::builder()
///             .display_name("tf-test MonitoringSubGroup")
///             .filter("resource.metadata.region=\"europe-west2\"")
///             .parent_name("${parent.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Group can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Group can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:monitoring/group:Group default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/group:Group default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:monitoring/group:Group default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// A user-assigned name for this group, used only for display
        /// purposes.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The filter used to determine which monitored resources
        /// belong to this group.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, the members of this group are considered to be a
        /// cluster. The system can perform additional analysis on
        /// groups that are clusters.
        #[builder(into, default)]
        pub is_cluster: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the group's parent, if it has one. The format is
        /// "projects/{project_id_or_number}/groups/{group_id}". For
        /// groups with no parent, parentName is the empty string, "".
        #[builder(into, default)]
        pub parent_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// A user-assigned name for this group, used only for display
        /// purposes.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The filter used to determine which monitored resources
        /// belong to this group.
        ///
        ///
        /// - - -
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// If true, the members of this group are considered to be a
        /// cluster. The system can perform additional analysis on
        /// groups that are clusters.
        pub is_cluster: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A unique identifier for this group. The format is
        /// "projects/{project_id_or_number}/groups/{group_id}".
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the group's parent, if it has one. The format is
        /// "projects/{project_id_or_number}/groups/{group_id}". For
        /// groups with no parent, parentName is the empty string, "".
        pub parent_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
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
        let display_name_binding = args.display_name.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let is_cluster_binding = args.is_cluster.get_output(context);
        let parent_name_binding = args.parent_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:monitoring/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isCluster".into(),
                    value: &is_cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentName".into(),
                    value: &parent_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupResult {
            display_name: o.get_field("displayName"),
            filter: o.get_field("filter"),
            is_cluster: o.get_field("isCluster"),
            name: o.get_field("name"),
            parent_name: o.get_field("parentName"),
            project: o.get_field("project"),
        }
    }
}
