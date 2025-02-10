/// Manages an Application Insights API key.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let authenticateSdkControlChannel = api_key::create(
///         "authenticateSdkControlChannel",
///         ApiKeyArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .name("tf-test-appinsights-authenticate-sdk-control-channel-api-key")
///             .read_permissions(vec!["agentconfig",])
///             .build_struct(),
///     );
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
///     let fullPermissions = api_key::create(
///         "fullPermissions",
///         ApiKeyArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .name("tf-test-appinsights-full-permissions-api-key")
///             .read_permissions(
///                 vec![
///                     "agentconfig", "aggregate", "api", "draft", "extendqueries",
///                     "search",
///                 ],
///             )
///             .write_permissions(vec!["annotations",])
///             .build_struct(),
///     );
///     let readTelemetry = api_key::create(
///         "readTelemetry",
///         ApiKeyArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .name("tf-test-appinsights-read-telemetry-api-key")
///             .read_permissions(
///                 vec!["aggregate", "api", "draft", "extendqueries", "search",],
///             )
///             .build_struct(),
///     );
///     let writeAnnotations = api_key::create(
///         "writeAnnotations",
///         ApiKeyArgs::builder()
///             .application_insights_id("${exampleInsights.id}")
///             .name("tf-test-appinsights-write-annotations-api-key")
///             .write_permissions(vec!["annotations",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Application Insights API keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appinsights/apiKey:ApiKey my_key /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Insights/components/instance1/apiKeys/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiKeyArgs {
        /// The ID of the Application Insights component on which the API key operates. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_insights_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Application Insights API key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the list of read permissions granted to the API key. Valid values are `agentconfig`, `aggregate`, `api`, `draft`, `extendqueries`, `search`. Please note these values are case sensitive. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub read_permissions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the list of write permissions granted to the API key. Valid values are `annotations`. Please note these values are case sensitive. Changing this forces a new resource to be created.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        #[builder(into, default)]
        pub write_permissions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ApiKeyResult {
        /// The API Key secret (Sensitive).
        pub api_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Application Insights component on which the API key operates. Changing this forces a new resource to be created.
        pub application_insights_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Application Insights API key. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the list of read permissions granted to the API key. Valid values are `agentconfig`, `aggregate`, `api`, `draft`, `extendqueries`, `search`. Please note these values are case sensitive. Changing this forces a new resource to be created.
        pub read_permissions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the list of write permissions granted to the API key. Valid values are `annotations`. Please note these values are case sensitive. Changing this forces a new resource to be created.
        ///
        /// > **Note:** At least one read or write permission must be defined.
        pub write_permissions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiKeyArgs,
    ) -> ApiKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_insights_id_binding = args
            .application_insights_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let read_permissions_binding = args.read_permissions.get_output(context);
        let write_permissions_binding = args.write_permissions.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appinsights/apiKey:ApiKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationInsightsId".into(),
                    value: application_insights_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readPermissions".into(),
                    value: read_permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "writePermissions".into(),
                    value: write_permissions_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiKeyResult {
            api_key: o.get_field("apiKey"),
            application_insights_id: o.get_field("applicationInsightsId"),
            name: o.get_field("name"),
            read_permissions: o.get_field("readPermissions"),
            write_permissions: o.get_field("writePermissions"),
        }
    }
}
