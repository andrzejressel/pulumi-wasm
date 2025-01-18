/// Manages a Sentinel Watchlist Item.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: example-workspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///   exampleLogAnalyticsWorkspaceOnboarding:
///     type: azure:sentinel:LogAnalyticsWorkspaceOnboarding
///     name: example
///     properties:
///       workspaceId: ${exampleAnalyticsWorkspace.id}
///   exampleWatchlist:
///     type: azure:sentinel:Watchlist
///     name: example
///     properties:
///       name: example-watchlist
///       logAnalyticsWorkspaceId: ${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}
///       displayName: example-wl
///       itemSearchKey: Key
///   exampleWatchlistItem:
///     type: azure:sentinel:WatchlistItem
///     name: example
///     properties:
///       name: 0aac6fa5-223e-49cf-9bfd-3554dc9d2b76
///       watchlistId: ${exampleWatchlist.id}
///       properties:
///         k1: v1
///         k2: v2
/// ```
///
/// ## Import
///
/// Sentinel Watchlist Items can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/watchlistItem:WatchlistItem example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/watchlists/list1/watchlistItems/item1
/// ```
///
pub mod watchlist_item {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WatchlistItemArgs {
        /// The name in UUID format which should be used for this Sentinel Watchlist Item. Changing this forces a new Sentinel Watchlist Item to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The key value pairs of the Sentinel Watchlist Item.
        #[builder(into)]
        pub properties: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Sentinel Watchlist that this Item resides in. Changing this forces a new Sentinel Watchlist Item to be created.
        #[builder(into)]
        pub watchlist_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WatchlistItemResult {
        /// The name in UUID format which should be used for this Sentinel Watchlist Item. Changing this forces a new Sentinel Watchlist Item to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The key value pairs of the Sentinel Watchlist Item.
        pub properties: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the Sentinel Watchlist that this Item resides in. Changing this forces a new Sentinel Watchlist Item to be created.
        pub watchlist_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WatchlistItemArgs) -> WatchlistItemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let properties_binding = args.properties.get_inner();
        let watchlist_id_binding = args.watchlist_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/watchlistItem:WatchlistItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "properties".into(),
                    value: &properties_binding,
                },
                register_interface::ObjectField {
                    name: "watchlistId".into(),
                    value: &watchlist_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "watchlistId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WatchlistItemResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            watchlist_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("watchlistId").unwrap(),
            ),
        }
    }
}
