/// Manages a Log Analytics (formally Operational Insights) Saved Search.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleSavedSearch = saved_search::create(
///         "exampleSavedSearch",
///         SavedSearchArgs::builder()
///             .category("exampleCategory")
///             .display_name("exampleDisplayName")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("exampleSavedSearch")
///             .query("exampleQuery")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Log Analytics Saved Searches can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/savedSearch:SavedSearch search1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/savedSearches/search1
/// ```
///
pub mod saved_search {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SavedSearchArgs {
        /// The category that the Saved Search will be listed under. Changing this forces a new resource to be created.
        #[builder(into)]
        pub category: pulumi_wasm_rust::Output<String>,
        /// The name that Saved Search will be displayed as. Changing this forces a new resource to be created.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The function alias if the query serves as a function. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub function_alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The function parameters if the query serves as a function. Changing this forces a new resource to be created. For more examples and proper syntax please refer to [this document](https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/functions/user-defined-functions).
        #[builder(into, default)]
        pub function_parameters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the ID of the Log Analytics Workspace that the Saved Search will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Log Analytics Saved Search. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The query expression for the saved search. Changing this forces a new resource to be created.
        #[builder(into)]
        pub query: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Logs Analytics Saved Search. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SavedSearchResult {
        /// The category that the Saved Search will be listed under. Changing this forces a new resource to be created.
        pub category: pulumi_wasm_rust::Output<String>,
        /// The name that Saved Search will be displayed as. Changing this forces a new resource to be created.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The function alias if the query serves as a function. Changing this forces a new resource to be created.
        pub function_alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The function parameters if the query serves as a function. Changing this forces a new resource to be created. For more examples and proper syntax please refer to [this document](https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/functions/user-defined-functions).
        pub function_parameters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the ID of the Log Analytics Workspace that the Saved Search will be associated with. Changing this forces a new resource to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Log Analytics Saved Search. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The query expression for the saved search. Changing this forces a new resource to be created.
        pub query: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Logs Analytics Saved Search. Changing this forces a new resource to be created.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SavedSearchArgs) -> SavedSearchResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let category_binding = args.category.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let function_alias_binding = args.function_alias.get_inner();
        let function_parameters_binding = args.function_parameters.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let query_binding = args.query.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/savedSearch:SavedSearch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "functionAlias".into(),
                    value: &function_alias_binding,
                },
                register_interface::ObjectField {
                    name: "functionParameters".into(),
                    value: &function_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "category".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "functionAlias".into(),
                },
                register_interface::ResultField {
                    name: "functionParameters".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "query".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SavedSearchResult {
            category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("category").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            function_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionAlias").unwrap(),
            ),
            function_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionParameters").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("query").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
