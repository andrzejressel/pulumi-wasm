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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod entry_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EntryGroupArgs {
        /// Entry group description, which can consist of several sentences or paragraphs that describe entry group contents.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A short name to identify the entry group, for example, "analytics data - jan 2011".
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the entry group to create. The id must begin with a letter or underscore,
        /// contain only English letters, numbers and underscores, and be at most 64 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub entry_group_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// EntryGroup location region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EntryGroupResult {
        /// Entry group description, which can consist of several sentences or paragraphs that describe entry group contents.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A short name to identify the entry group, for example, "analytics data - jan 2011".
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the entry group to create. The id must begin with a letter or underscore,
        /// contain only English letters, numbers and underscores, and be at most 64 characters.
        ///
        ///
        /// - - -
        pub entry_group_id: pulumi_wasm_rust::Output<String>,
        /// The resource name of the entry group in URL format. Example: projects/{project}/locations/{location}/entryGroups/{entryGroupId}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// EntryGroup location region.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EntryGroupArgs) -> EntryGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let entry_group_id_binding = args.entry_group_id.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datacatalog/entryGroup:EntryGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "entryGroupId".into(),
                    value: &entry_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "entryGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EntryGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            entry_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entryGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
