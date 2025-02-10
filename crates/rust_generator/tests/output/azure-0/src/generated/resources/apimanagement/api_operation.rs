/// Manages an API Operation within an API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApiOperation:
///     type: azure:apimanagement:ApiOperation
///     name: example
///     properties:
///       operationId: user-delete
///       apiName: ${example.name}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       displayName: Delete User Operation
///       method: DELETE
///       urlTemplate: /users/{id}/delete
///       description: This can only be done by the logged in user.
///       templateParameters:
///         - name: id
///           type: number
///           required: true
///       responses:
///         - statusCode: 200
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
/// API Management API Operation's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiOperation:ApiOperation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/apis/api1/operations/operation1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_operation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiOperationArgs {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API within the API Management Service where this API Operation should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for this API Operation, which may include HTML formatting tags.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this API Management Operation.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The HTTP Method used for this API Management Operation, like `GET`, `DELETE`, `PUT` or `POST` - but not limited to these values.
        #[builder(into)]
        pub method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for this API Operation. Changing this forces a new resource to be created.
        #[builder(into)]
        pub operation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `request` block as defined below.
        #[builder(into, default)]
        pub request: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apimanagement::ApiOperationRequest>,
        >,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `response` blocks as defined below.
        #[builder(into, default)]
        pub responses: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::ApiOperationResponse>>,
        >,
        /// One or more `template_parameter` blocks as defined below. Required if `url_template` contains one or more parameters.
        #[builder(into, default)]
        pub template_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::apimanagement::ApiOperationTemplateParameter>,
            >,
        >,
        /// The relative URL Template identifying the target resource for this operation, which may include parameters.
        #[builder(into)]
        pub url_template: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiOperationResult {
        /// The Name of the API Management Service where the API exists. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API within the API Management Service where this API Operation should be created. Changing this forces a new resource to be created.
        pub api_name: pulumi_gestalt_rust::Output<String>,
        /// A description for this API Operation, which may include HTML formatting tags.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Display Name for this API Management Operation.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The HTTP Method used for this API Management Operation, like `GET`, `DELETE`, `PUT` or `POST` - but not limited to these values.
        pub method: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for this API Operation. Changing this forces a new resource to be created.
        pub operation_id: pulumi_gestalt_rust::Output<String>,
        /// A `request` block as defined below.
        pub request: pulumi_gestalt_rust::Output<
            super::super::types::apimanagement::ApiOperationRequest,
        >,
        /// The Name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `response` blocks as defined below.
        pub responses: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::apimanagement::ApiOperationResponse>>,
        >,
        /// One or more `template_parameter` blocks as defined below. Required if `url_template` contains one or more parameters.
        pub template_parameters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::apimanagement::ApiOperationTemplateParameter>,
            >,
        >,
        /// The relative URL Template identifying the target resource for this operation, which may include parameters.
        pub url_template: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiOperationArgs,
    ) -> ApiOperationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let api_name_binding = args.api_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let method_binding = args.method.get_output(context);
        let operation_id_binding = args.operation_id.get_output(context);
        let request_binding = args.request.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let responses_binding = args.responses.get_output(context);
        let template_parameters_binding = args.template_parameters.get_output(context);
        let url_template_binding = args.url_template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiOperation:ApiOperation".into(),
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
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationId".into(),
                    value: operation_id_binding.get_id(),
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
                    name: "responses".into(),
                    value: responses_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateParameters".into(),
                    value: template_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlTemplate".into(),
                    value: url_template_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiOperationResult {
            api_management_name: o.get_field("apiManagementName"),
            api_name: o.get_field("apiName"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            method: o.get_field("method"),
            operation_id: o.get_field("operationId"),
            request: o.get_field("request"),
            resource_group_name: o.get_field("resourceGroupName"),
            responses: o.get_field("responses"),
            template_parameters: o.get_field("templateParameters"),
            url_template: o.get_field("urlTemplate"),
        }
    }
}
