/// Manages an Application Insights WebTest.
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
///     let exampleInsights = insights::create(
///         "exampleInsights",
///         InsightsArgs::builder()
///             .application_type("web")
///             .location("${example.location}")
///             .name("tf-test-appinsights")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleWebTest = web_test::create(
///         "exampleWebTest",
///         WebTestArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .configuration(
///                 "<WebTest Name=\"WebTest1\" Id=\"ABD48585-0831-40CB-9069-682EA6BB3583\" Enabled=\"True\" CssProjectStructure=\"\" CssIteration=\"\" Timeout=\"0\" WorkItemIds=\"\" xmlns=\"http://microsoft.com/schemas/VisualStudio/TeamTest/2010\" Description=\"\" CredentialUserName=\"\" CredentialPassword=\"\" PreAuthenticate=\"True\" Proxy=\"default\" StopOnError=\"False\" RecordedResultFile=\"\" ResultsLocale=\"\">\n  <Items>\n    <Request Method=\"GET\" Guid=\"a5f10126-e4cd-570d-961c-cea43999a200\" Version=\"1.1\" Url=\"http://microsoft.com\" ThinkTime=\"0\" Timeout=\"300\" ParseDependentRequests=\"True\" FollowRedirects=\"True\" RecordResult=\"True\" Cache=\"False\" ResponseTimeGoal=\"0\" Encoding=\"utf-8\" ExpectedHttpStatusCode=\"200\" ExpectedResponseUrl=\"\" ReportingName=\"\" IgnoreHttpStatusCode=\"False\" />\n  </Items>\n</WebTest>\n",
///             )
///             .enabled(true)
///             .frequency(300)
///             .geo_locations(vec!["us-tx-sn1-azr", "us-il-ch1-azr",])
///             .kind("ping")
///             .location("${exampleInsights.location}")
///             .name("tf-test-appinsights-webtest")
///             .resource_group_name("${example.name}")
///             .timeout(60)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights Web Tests can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/webTest:WebTest my_test /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Insights/webTests/my_test
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_test {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebTestArgs {
        /// The ID of the Application Insights component on which the WebTest operates. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An XML configuration specification for a WebTest ([see here for more information](https://docs.microsoft.com/rest/api/application-insights/webtests/createorupdate/)).
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Purpose/user defined descriptive test for this WebTest.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the test actively being monitored.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Interval in seconds between test runs for this WebTest. Valid options are `300`, `600` and `900`. Defaults to `300`.
        #[builder(into, default)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A list of where to physically run the tests from to give global coverage for accessibility of your application.
        ///
        /// > **Note:** [Valid options for geo locations are described here](https://docs.microsoft.com/azure/azure-monitor/app/monitor-web-app-availability#location-population-tags)
        #[builder(into)]
        pub geo_locations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The kind of web test that this web test watches. Choices are `ping` and `multistep`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. It needs to correlate with location of parent resource (azurerm_application_insights).
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Application Insights WebTest. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Application Insights WebTest. Changing this forces a new resource
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Allow for retries should this WebTest fail.
        #[builder(into, default)]
        pub retry_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Seconds until this WebTest will timeout and fail. Default is `30`.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct WebTestResult {
        /// The ID of the Application Insights component on which the WebTest operates. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// An XML configuration specification for a WebTest ([see here for more information](https://docs.microsoft.com/rest/api/application-insights/webtests/createorupdate/)).
        pub configuration: pulumi_gestalt_rust::Output<String>,
        /// Purpose/user defined descriptive test for this WebTest.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is the test actively being monitored.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Interval in seconds between test runs for this WebTest. Valid options are `300`, `600` and `900`. Defaults to `300`.
        pub frequency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A list of where to physically run the tests from to give global coverage for accessibility of your application.
        ///
        /// > **Note:** [Valid options for geo locations are described here](https://docs.microsoft.com/azure/azure-monitor/app/monitor-web-app-availability#location-population-tags)
        pub geo_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The kind of web test that this web test watches. Choices are `ping` and `multistep`. Changing this forces a new resource to be created.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. It needs to correlate with location of parent resource (azurerm_application_insights).
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Application Insights WebTest. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Application Insights WebTest. Changing this forces a new resource
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Allow for retries should this WebTest fail.
        pub retry_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub synthetic_monitor_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Seconds until this WebTest will timeout and fail. Default is `30`.
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WebTestArgs,
    ) -> WebTestResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_insights_id_binding_1 = args
            .application_insights_id
            .get_output(context);
        let application_insights_id_binding = application_insights_id_binding_1
            .get_inner();
        let configuration_binding_1 = args.configuration.get_output(context);
        let configuration_binding = configuration_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let frequency_binding_1 = args.frequency.get_output(context);
        let frequency_binding = frequency_binding_1.get_inner();
        let geo_locations_binding_1 = args.geo_locations.get_output(context);
        let geo_locations_binding = geo_locations_binding_1.get_inner();
        let kind_binding_1 = args.kind.get_output(context);
        let kind_binding = kind_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let retry_enabled_binding_1 = args.retry_enabled.get_output(context);
        let retry_enabled_binding = retry_enabled_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeout_binding_1 = args.timeout.get_output(context);
        let timeout_binding = timeout_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appinsights/webTest:WebTest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationInsightsId".into(),
                    value: &application_insights_id_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding,
                },
                register_interface::ObjectField {
                    name: "geoLocations".into(),
                    value: &geo_locations_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retryEnabled".into(),
                    value: &retry_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebTestResult {
            application_insights_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationInsightsId"),
            ),
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frequency: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frequency"),
            ),
            geo_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("geoLocations"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retry_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retryEnabled"),
            ),
            synthetic_monitor_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("syntheticMonitorId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeout"),
            ),
        }
    }
}
