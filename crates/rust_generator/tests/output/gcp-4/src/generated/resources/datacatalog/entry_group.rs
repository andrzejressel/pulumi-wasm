/// An EntryGroup resource represents a logical grouping of zero or more Data Catalog Entry resources.
///
///
/// To get more information about EntryGroup, see:
///
/// * [API documentation](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-catalog/docs)
///
/// ## Example Usage
///
/// ### Data Catalog Entry Group Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicEntryGroup = entry_group::create(
///         "basicEntryGroup",
///         EntryGroupArgs::builder().entry_group_id("my_group").build_struct(),
///     );
/// }
/// ```
/// ### Data Catalog Entry Group Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicEntryGroup = entry_group::create(
///         "basicEntryGroup",
///         EntryGroupArgs::builder()
///             .description("example entry group")
///             .display_name("entry group")
///             .entry_group_id("my_group")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// EntryGroup can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EntryGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datacatalog/entryGroup:EntryGroup default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entry_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryGroupArgs {
        /// Entry group description, which can consist of several sentences or paragraphs that describe entry group contents.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short name to identify the entry group, for example, "analytics data - jan 2011".
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the entry group to create. The id must begin with a letter or underscore,
        /// contain only English letters, numbers and underscores, and be at most 64 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub entry_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EntryGroup location region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntryGroupResult {
        /// Entry group description, which can consist of several sentences or paragraphs that describe entry group contents.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A short name to identify the entry group, for example, "analytics data - jan 2011".
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The id of the entry group to create. The id must begin with a letter or underscore,
        /// contain only English letters, numbers and underscores, and be at most 64 characters.
        ///
        ///
        /// - - -
        pub entry_group_id: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the entry group in URL format. Example: projects/{project}/locations/{location}/entryGroups/{entryGroupId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// EntryGroup location region.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EntryGroupArgs,
    ) -> EntryGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let entry_group_id_binding = args.entry_group_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datacatalog/entryGroup:EntryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entryGroupId".into(),
                    value: entry_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EntryGroupResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            entry_group_id: o.get_field("entryGroupId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
