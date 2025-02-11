/// Manages an API Management Gateway.
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
///     let exampleGateway = gateway::create(
///         "exampleGateway",
///         GatewayArgs::builder()
///             .api_management_id("${exampleService.id}")
///             .description("Example API Management gateway")
///             .location_data(
///                 GatewayLocationData::builder()
///                     .city("example city")
///                     .district("example district")
///                     .name("example name")
///                     .region("example region")
///                     .build_struct(),
///             )
///             .name("example-gateway")
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
///             .sku_name("Consumption_0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/gateway:Gateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/gateways/gateway1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayArgs {
        /// The ID of the API Management Resource in which the gateway will be created. Changing this forces a new API Management Gateway resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the API Management Gateway.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `location_data` block as documented below.
        #[builder(into)]
        pub location_data: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apimanagement::GatewayLocationData,
        >,
        /// The name which should be used for the API Management Gateway. Changing this forces a new API Management Gateway to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewayResult {
        /// The ID of the API Management Resource in which the gateway will be created. Changing this forces a new API Management Gateway resource to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the API Management Gateway.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `location_data` block as documented below.
        pub location_data: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::GatewayLocationData,
        >,
        /// The name which should be used for the API Management Gateway. Changing this forces a new API Management Gateway to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GatewayArgs,
    ) -> GatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let location_data_binding = args.location_data.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/gateway:Gateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationData".into(),
                    value: &location_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayResult {
            api_management_id: o.get_field("apiManagementId"),
            description: o.get_field("description"),
            location_data: o.get_field("locationData"),
            name: o.get_field("name"),
        }
    }
}
