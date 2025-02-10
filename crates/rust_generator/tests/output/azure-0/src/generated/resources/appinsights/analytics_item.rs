/// Manages an Application Insights Analytics Item component.
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
///             .name("tf-test")
///             .build_struct(),
///     );
///     let exampleAnalyticsItem = analytics_item::create(
///         "exampleAnalyticsItem",
///         AnalyticsItemArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .content("requests //simple example query")
///             .name("testquery")
///             .scope("shared")
///             .type_("query")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("tf-test-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights Analytics Items can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/analyticsItem:AnalyticsItem example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Insights/components/mycomponent1/analyticsItems/11111111-1111-1111-1111-111111111111
/// ```
///
/// To find the Analytics Item ID you can query the REST API using the [`az rest` CLI command](https://docs.microsoft.com/cli/azure/reference-index?view=azure-cli-latest#az-rest), e.g.
///
/// az rest --method GET --uri "https://management.azure.com/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/microsoft.insights/components/appinsightstest/analyticsItems?api-version=2015-05-01"
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod analytics_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyticsItemArgs {
        /// The ID of the Application Insights component on which the Analytics Item exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The content for the Analytics Item, for example the query text if `type` is `query`.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The alias to use for the function. Required when `type` is `function`.
        #[builder(into, default)]
        pub function_alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Application Insights Analytics Item. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope for the Analytics Item. Can be `shared` or `user`. Changing this forces a new resource to be created. Must be `shared` for functions.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Analytics Item to create. Can be one of `query`, `function`, `folder`, `recent`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AnalyticsItemResult {
        /// The ID of the Application Insights component on which the Analytics Item exists. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// The content for the Analytics Item, for example the query text if `type` is `query`.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The alias to use for the function. Required when `type` is `function`.
        pub function_alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Application Insights Analytics Item. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The scope for the Analytics Item. Can be `shared` or `user`. Changing this forces a new resource to be created. Must be `shared` for functions.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// A string containing the time the Analytics Item was created.
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// A string containing the time the Analytics Item was last modified.
        pub time_modified: pulumi_gestalt_rust::Output<String>,
        /// The type of Analytics Item to create. Can be one of `query`, `function`, `folder`, `recent`. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A string indicating the version of the query format
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnalyticsItemArgs,
    ) -> AnalyticsItemResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context);
        let content_binding = args.content.get_output(context);
        let function_alias_binding = args.function_alias.get_output(context);
        let name_binding = args.name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/analyticsItem:AnalyticsItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsId".into(),
                    value: application_insights_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionAlias".into(),
                    value: function_alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnalyticsItemResult {
            application_insights_id: o.get_field("applicationInsightsId"),
            content: o.get_field("content"),
            function_alias: o.get_field("functionAlias"),
            name: o.get_field("name"),
            scope: o.get_field("scope"),
            time_created: o.get_field("timeCreated"),
            time_modified: o.get_field("timeModified"),
            type_: o.get_field("type"),
            version: o.get_field("version"),
        }
    }
}
