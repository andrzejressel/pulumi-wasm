/// Manages a Sentinel Watchlist.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod watchlist {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WatchlistArgs {
        /// The default duration in ISO8601 duration form of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub default_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key used to optimize query performance when using Watchlist for joins with other data. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub item_search_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a list of labels related to this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace where this Sentinel Watchlist resides in. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WatchlistResult {
        /// The default duration in ISO8601 duration form of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub default_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The key used to optimize query performance when using Watchlist for joins with other data. Changing this forces a new Sentinel Watchlist to be created.
        pub item_search_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of labels related to this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub labels: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Log Analytics Workspace where this Sentinel Watchlist resides in. Changing this forces a new Sentinel Watchlist to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Sentinel Watchlist. Changing this forces a new Sentinel Watchlist to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WatchlistArgs,
    ) -> WatchlistResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_duration_binding = args.default_duration.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let item_search_key_binding = args.item_search_key.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/watchlist:Watchlist".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultDuration".into(),
                    value: &default_duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "itemSearchKey".into(),
                    value: &item_search_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WatchlistResult {
            default_duration: o.get_field("defaultDuration"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            item_search_key: o.get_field("itemSearchKey"),
            labels: o.get_field("labels"),
            log_analytics_workspace_id: o.get_field("logAnalyticsWorkspaceId"),
            name: o.get_field("name"),
        }
    }
}
