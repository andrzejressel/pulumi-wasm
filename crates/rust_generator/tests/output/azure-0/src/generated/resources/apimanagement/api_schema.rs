/// Manages an API Schema within an API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApiSchema:
///     type: azure:apimanagement:ApiSchema
///     name: example
///     properties:
///       apiName: ${example.name}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       schemaId: example-schema
///       contentType: application/vnd.ms-azure-apim.xsd+xml
///       value:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: api_management_api_schema.xml
///           return: result
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getApi
///       arguments:
///         name: search-api
///         apiManagementName: search-api-management
///         resourceGroupName: search-service
///         revision: '2'
/// ```
///
/// ## Import
///
/// API Management API Schema's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiSchema:ApiSchema example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/apis/api1/schemas/schema1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiSchemaArgs {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API within the API Management Service where this API Schema should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Types definitions. Used for Swagger/OpenAPI v2/v3 schemas only.
        #[builder(into, default)]
        pub components: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content type of the API Schema.
        #[builder(into)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Types definitions. Used for Swagger/OpenAPI v1 schemas only.
        #[builder(into, default)]
        pub definitions: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for this API Schema. Changing this forces a new resource to be created.
        #[builder(into)]
        pub schema_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The JSON escaped string defining the document representing the Schema.
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiSchemaResult {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API within the API Management Service where this API Schema should be created. Changing this forces a new resource to be created.
        pub api_name: pulumi_gestalt_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v2/v3 schemas only.
        pub components: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content type of the API Schema.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// Types definitions. Used for Swagger/OpenAPI v1 schemas only.
        pub definitions: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for this API Schema. Changing this forces a new resource to be created.
        pub schema_id: pulumi_gestalt_rust::Output<String>,
        /// The JSON escaped string defining the document representing the Schema.
        pub value: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiSchemaArgs,
    ) -> ApiSchemaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let api_name_binding = args.api_name.get_output(context);
        let components_binding = args.components.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let definitions_binding = args.definitions.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let schema_id_binding = args.schema_id.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiSchema:ApiSchema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiName".into(),
                    value: api_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "components".into(),
                    value: components_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definitions".into(),
                    value: definitions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaId".into(),
                    value: schema_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiSchemaResult {
            api_management_name: o.get_field("apiManagementName"),
            api_name: o.get_field("apiName"),
            components: o.get_field("components"),
            content_type: o.get_field("contentType"),
            definitions: o.get_field("definitions"),
            resource_group_name: o.get_field("resourceGroupName"),
            schema_id: o.get_field("schemaId"),
            value: o.get_field("value"),
        }
    }
}
