/// Manages a Sentinel Watchlist.
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
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
///     let exampleWatchlist = watchlist::create(
///         "exampleWatchlist",
///         WatchlistArgs::builder()
///             .display_name("example-wl")
///             .item_search_key("Key")
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example-watchlist")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Sentinel Watchlists can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/watchlist:Watchlist example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/watchlists/list1
/// ```
///
pub mod watchlist {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WatchlistArgs {
        /// The default duration in ISO8601 duration form of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub default_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The key used to optimize query performance when using Watchlist for joins with other data. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub item_search_key: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of labels related to this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace where this Sentinel Watchlist resides in. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WatchlistResult {
        /// The default duration in ISO8601 duration form of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub default_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The key used to optimize query performance when using Watchlist for joins with other data. Changing this forces a new Sentinel Watchlist to be created.
        pub item_search_key: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of labels related to this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub labels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace where this Sentinel Watchlist resides in. Changing this forces a new Sentinel Watchlist to be created.
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WatchlistArgs) -> WatchlistResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_duration_binding = args.default_duration.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let item_search_key_binding = args.item_search_key.get_inner();
        let labels_binding = args.labels.get_inner();
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/watchlist:Watchlist".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultDuration".into(),
                    value: &default_duration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "itemSearchKey".into(),
                    value: &item_search_key_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultDuration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "itemSearchKey".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WatchlistResult {
            default_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultDuration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            item_search_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("itemSearchKey").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
