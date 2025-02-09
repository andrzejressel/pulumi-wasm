/// Manages a Application Insights Standard WebTest.
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
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStandardWebTest = standard_web_test::create(
///         "exampleStandardWebTest",
///         StandardWebTestArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .geo_locations(vec!["example",])
///             .location("West Europe")
///             .name("example-test")
///             .request(
///                 StandardWebTestRequest::builder()
///                     .url("http://www.example.com")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights Standard WebTests can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/standardWebTest:StandardWebTest example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.Insights/webTests/appinsightswebtest
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod standard_web_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardWebTestArgs {
        /// The ID of the Application Insights instance on which the WebTest operates. Changing this forces a new Application Insights Standard WebTest to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Purpose/user defined descriptive test for this WebTest.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the WebTest be enabled?
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Interval in seconds between test runs for this WebTest. Valid options are `300`, `600` and `900`. Defaults to `300`.
        #[builder(into, default)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies a list of where to physically run the tests from to give global coverage for accessibility of your application.
        ///
        /// > **Note:** [Valid options for geo locations are described here](https://docs.microsoft.com/azure/azure-monitor/app/monitor-web-app-availability#location-population-tags)
        #[builder(into)]
        pub geo_locations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The Azure Region where the Application Insights Standard WebTest should exist. Changing this forces a new Application Insights Standard WebTest to be created. It needs to correlate with location of the parent resource (azurerm_application_insights)
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Application Insights Standard WebTest. Changing this forces a new Application Insights Standard WebTest to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `request` block as defined below.
        #[builder(into)]
        pub request: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appinsights::StandardWebTestRequest,
        >,
        /// The name of the Resource Group where the Application Insights Standard WebTest should exist. Changing this forces a new Application Insights Standard WebTest to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the retry on WebTest failure be enabled?
        #[builder(into, default)]
        pub retry_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags which should be assigned to the Application Insights Standard WebTest.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Seconds until this WebTest will timeout and fail. Default is `30`.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A `validation_rules` block as defined below.
        #[builder(into, default)]
        pub validation_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appinsights::StandardWebTestValidationRules>,
        >,
    }
    #[allow(dead_code)]
    pub struct StandardWebTestResult {
        /// The ID of the Application Insights instance on which the WebTest operates. Changing this forces a new Application Insights Standard WebTest to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// Purpose/user defined descriptive test for this WebTest.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the WebTest be enabled?
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Interval in seconds between test runs for this WebTest. Valid options are `300`, `600` and `900`. Defaults to `300`.
        pub frequency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies a list of where to physically run the tests from to give global coverage for accessibility of your application.
        ///
        /// > **Note:** [Valid options for geo locations are described here](https://docs.microsoft.com/azure/azure-monitor/app/monitor-web-app-availability#location-population-tags)
        pub geo_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Azure Region where the Application Insights Standard WebTest should exist. Changing this forces a new Application Insights Standard WebTest to be created. It needs to correlate with location of the parent resource (azurerm_application_insights)
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Application Insights Standard WebTest. Changing this forces a new Application Insights Standard WebTest to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `request` block as defined below.
        pub request: pulumi_gestalt_rust::Output<
            super::super::types::appinsights::StandardWebTestRequest,
        >,
        /// The name of the Resource Group where the Application Insights Standard WebTest should exist. Changing this forces a new Application Insights Standard WebTest to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Should the retry on WebTest failure be enabled?
        pub retry_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Unique ID of this WebTest. This is typically the same value as the Name field.
        pub synthetic_monitor_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Application Insights Standard WebTest.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Seconds until this WebTest will timeout and fail. Default is `30`.
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A `validation_rules` block as defined below.
        pub validation_rules: pulumi_gestalt_rust::Output<
            Option<super::super::types::appinsights::StandardWebTestValidationRules>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StandardWebTestArgs,
    ) -> StandardWebTestResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let geo_locations_binding = args.geo_locations.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let request_binding = args.request.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let retry_enabled_binding = args.retry_enabled.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let validation_rules_binding = args.validation_rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/standardWebTest:StandardWebTest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsId".into(),
                    value: application_insights_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: frequency_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "geoLocations".into(),
                    value: geo_locations_binding.get_id(),
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
                    name: "request".into(),
                    value: request_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryEnabled".into(),
                    value: retry_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationRules".into(),
                    value: validation_rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StandardWebTestResult {
            application_insights_id: o.get_field("applicationInsightsId"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            frequency: o.get_field("frequency"),
            geo_locations: o.get_field("geoLocations"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            request: o.get_field("request"),
            resource_group_name: o.get_field("resourceGroupName"),
            retry_enabled: o.get_field("retryEnabled"),
            synthetic_monitor_id: o.get_field("syntheticMonitorId"),
            tags: o.get_field("tags"),
            timeout: o.get_field("timeout"),
            validation_rules: o.get_field("validationRules"),
        }
    }
}
