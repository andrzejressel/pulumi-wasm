/// Manages an API Version Set within an API Management Service.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleApiVersionSet = api_version_set::create(
///         "exampleApiVersionSet",
///         ApiVersionSetArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .display_name("ExampleAPIVersionSet")
///             .name("example-apimapi-1_0_0")
///             .resource_group_name("${example.name}")
///             .versioning_scheme("Segment")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("pub1@email.com")
///             .publisher_name("pub1")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Version Set can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiVersionSet:ApiVersionSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apiVersionSets/set1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_version_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiVersionSetArgs {
        /// The name of the API Management Service in which the API Version Set should exist. May only contain alphanumeric characters and dashes up to 50 characters in length. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of API Version Set.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of this API Version Set.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Version Set. May only contain alphanumeric characters and dashes up to 80 characters in length. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the parent API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Header which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Header`.
        #[builder(into, default)]
        pub version_header_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Query String which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Query`.
        #[builder(into, default)]
        pub version_query_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies where in an Inbound HTTP Request that the API Version should be read from. Possible values are `Header`, `Query` and `Segment`.
        #[builder(into)]
        pub versioning_scheme: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiVersionSetResult {
        /// The name of the API Management Service in which the API Version Set should exist. May only contain alphanumeric characters and dashes up to 50 characters in length. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The description of API Version Set.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of this API Version Set.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Version Set. May only contain alphanumeric characters and dashes up to 80 characters in length. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the parent API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Header which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Header`.
        pub version_header_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Query String which should be read from Inbound Requests which defines the API Version.
        ///
        /// > **NOTE:** This must be specified when `versioning_scheme` is set to `Query`.
        pub version_query_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies where in an Inbound HTTP Request that the API Version should be read from. Possible values are `Header`, `Query` and `Segment`.
        pub versioning_scheme: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiVersionSetArgs,
    ) -> ApiVersionSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let version_header_name_binding = args.version_header_name.get_output(context);
        let version_query_name_binding = args.version_query_name.get_output(context);
        let versioning_scheme_binding = args.versioning_scheme.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiVersionSet:ApiVersionSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionHeaderName".into(),
                    value: version_header_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionQueryName".into(),
                    value: version_query_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versioningScheme".into(),
                    value: versioning_scheme_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiVersionSetResult {
            api_management_name: o.get_field("apiManagementName"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            version_header_name: o.get_field("versionHeaderName"),
            version_query_name: o.get_field("versionQueryName"),
            versioning_scheme: o.get_field("versioningScheme"),
        }
    }
}
