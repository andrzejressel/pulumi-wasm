/// Describes a group of resources to read log entries from
///
///
/// To get more information about LogScope, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/projects.locations.logScopes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/logging/docs/apis)
///
/// ## Example Usage
///
/// ### Logging Log Scope Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingLogScope = log_scope::create(
///         "loggingLogScope",
///         LogScopeArgs::builder()
///             .description("A log scope configured with Terraform")
///             .location("global")
///             .name("projects/my-project-name/locations/global/logScopes/my-log-scope")
///             .parent("projects/my-project-name")
///             .resource_names(
///                 vec![
///                     "projects/my-project-name",
///                     "projects/my-project-name/locations/global/buckets/_Default/views/view1",
///                     "projects/my-project-name/locations/global/buckets/_Default/views/view2",
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// LogScope can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/logScopes/{{name}}`
///
/// When using the `pulumi import` command, LogScope can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/logScope:LogScope default {{parent}}/locations/{{location}}/logScopes/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogScopeArgs {
        /// Describes this log scopes.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the resource. The only supported location is global so far.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name of the log scope. For example: \`projects/my-project/locations/global/logScopes/my-log-scope\`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the resource.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Names of one or more parent resources : *  \`projects/[PROJECT_ID]\` May alternatively be one or more views : * \`projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]\` A log scope can include a maximum of 50 projects and a maximum of 100 resources in total.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub resource_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct LogScopeResult {
        /// Output only. The creation timestamp of the log scopes.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Describes this log scopes.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the resource. The only supported location is global so far.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the log scope. For example: \`projects/my-project/locations/global/logScopes/my-log-scope\`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Names of one or more parent resources : *  \`projects/[PROJECT_ID]\` May alternatively be one or more views : * \`projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]\` A log scope can include a maximum of 50 projects and a maximum of 100 resources in total.
        ///
        ///
        /// - - -
        pub resource_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Output only. The last update timestamp of the log scopes.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogScopeArgs,
    ) -> LogScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let resource_names_binding = args.resource_names.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/logScope:LogScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceNames".into(),
                    value: resource_names_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogScopeResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            resource_names: o.get_field("resourceNames"),
            update_time: o.get_field("updateTime"),
        }
    }
}
