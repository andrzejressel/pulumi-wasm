/// Manages a API Management API Operation Tag.
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
///   exampleApiOperationTag:
///     type: azure:apimanagement:ApiOperationTag
///     name: example
///     properties:
///       name: example-Tag
///       apiOperationId: ${exampleApiOperation.id}
///       displayName: example-Tag
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
/// API Management API Operation Tags can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiOperationTag:ApiOperationTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/api1/operations/operation1/tags/tag1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_operation_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiOperationTagArgs {
        /// The ID of the API Management API Operation. Changing this forces a new API Management API Operation Tag to be created.
        #[builder(into)]
        pub api_operation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name of the API Management API Operation Tag.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this API Management API Operation Tag. Changing this forces a new API Management API Operation Tag to be created. The name must be unique in the API Management Service.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiOperationTagResult {
        /// The ID of the API Management API Operation. Changing this forces a new API Management API Operation Tag to be created.
        pub api_operation_id: pulumi_gestalt_rust::Output<String>,
        /// The display name of the API Management API Operation Tag.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this API Management API Operation Tag. Changing this forces a new API Management API Operation Tag to be created. The name must be unique in the API Management Service.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiOperationTagArgs,
    ) -> ApiOperationTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_operation_id_binding = args.api_operation_id.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiOperationTag:ApiOperationTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiOperationId".into(),
                    value: api_operation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiOperationTagResult {
            api_operation_id: o.get_field("apiOperationId"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
        }
    }
}
