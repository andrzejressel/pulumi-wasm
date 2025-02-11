/// Manages the Assignment of an API Management API Tag to an API.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleApi:
///     type: azure:apimanagement:Api
///     name: example
///     properties:
///       name: example-api
///       resourceGroupName: ${exampleResourceGroup.name}
///       apiManagementName: ${example.name}
///       revision: '1'
///   exampleTag:
///     type: azure:apimanagement:Tag
///     name: example
///     properties:
///       apiManagementId: ${example.id}
///       name: example-tag
///   exampleApiTag:
///     type: azure:apimanagement:ApiTag
///     name: example
///     properties:
///       apiId: ${exampleApi.id}
///       name: ${exampleTag.name}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-apim
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// API Management API Tags can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiTag:ApiTag example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/api1/tags/tag1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiTagArgs {
        /// The ID of the API Management API. Changing this forces a new API Management API Tag to be created.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the tag. It must be known in the API Management instance. Changing this forces a new API Management API Tag to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiTagResult {
        /// The ID of the API Management API. Changing this forces a new API Management API Tag to be created.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the tag. It must be known in the API Management instance. Changing this forces a new API Management API Tag to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTagArgs,
    ) -> ApiTagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiTag:ApiTag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiTagResult {
            api_id: o.get_field("apiId"),
            name: o.get_field("name"),
        }
    }
}
