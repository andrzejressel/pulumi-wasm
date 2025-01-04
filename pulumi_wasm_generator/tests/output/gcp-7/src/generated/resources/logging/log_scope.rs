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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod log_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogScopeArgs {
        /// Describes this log scopes.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the resource. The only supported location is global so far.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the log scope. For example: \`projects/my-project/locations/global/logScopes/my-log-scope\`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parent of the resource.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// Names of one or more parent resources : *  \`projects/[PROJECT_ID]\` May alternatively be one or more views : * \`projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]\` A log scope can include a maximum of 50 projects and a maximum of 100 resources in total.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub resource_names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct LogScopeResult {
        /// Output only. The creation timestamp of the log scopes.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Describes this log scopes.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the resource. The only supported location is global so far.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the log scope. For example: \`projects/my-project/locations/global/logScopes/my-log-scope\`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the resource.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Names of one or more parent resources : *  \`projects/[PROJECT_ID]\` May alternatively be one or more views : * \`projects/[PROJECT_ID]/locations/[LOCATION_ID]/buckets/[BUCKET_ID]/views/[VIEW_ID]\` A log scope can include a maximum of 50 projects and a maximum of 100 resources in total.
        ///
        ///
        /// - - -
        pub resource_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Output only. The last update timestamp of the log scopes.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogScopeArgs) -> LogScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let parent_binding = args.parent.get_inner();
        let resource_names_binding = args.resource_names.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/logScope:LogScope".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "resourceNames".into(),
                    value: &resource_names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "resourceNames".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogScopeResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            resource_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceNames").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
