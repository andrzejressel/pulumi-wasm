/// Manages an API Tag Description within an API Management Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       publisherName: My Company
///       publisherEmail: company@terraform.io
///       skuName: Developer_1
///   exampleApi:
///     type: azure:apimanagement:Api
///     name: example
///     properties:
///       name: example-api
///       resourceGroupName: ${example.name}
///       apiManagementName: ${exampleService.name}
///       revision: '1'
///       displayName: Example API
///       path: example
///       protocols:
///         - https
///       import:
///         contentFormat: swagger-link-json
///         contentValue: https://raw.githubusercontent.com/hashicorp/terraform-provider-azurerm/refs/heads/main/internal/services/apimanagement/testdata/api_management_api_swagger.json
///   exampleTag:
///     type: azure:apimanagement:Tag
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       name: example-Tag
///   exampleApiTagDescription:
///     type: azure:apimanagement:ApiTagDescription
///     name: example
///     properties:
///       apiTagId: ${exampleTag.id}
///       description: This is an example description
///       externalDocsUrl: https://registry.terraform.io/providers/hashicorp/azurerm/latest/docs
///       externalDocsDescription: This is an example external docs description
/// ```
///
/// ## Import
///
/// API Management API Schema's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiTagDescription:ApiTagDescription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/apis/api1/tagDescriptions/tagDescriptionId1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_tag_description {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiTagDescriptionArgs {
        /// The The ID of the API Management API Tag. Changing this forces a new API Management API Tag Description to be created.
        #[builder(into)]
        pub api_tag_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the Tag.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the external documentation resources describing the tag.
        #[builder(into, default)]
        pub external_documentation_description: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The URL of external documentation resources describing the tag.
        #[builder(into, default)]
        pub external_documentation_url: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApiTagDescriptionResult {
        /// The The ID of the API Management API Tag. Changing this forces a new API Management API Tag Description to be created.
        pub api_tag_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the Tag.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the external documentation resources describing the tag.
        pub external_documentation_description: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The URL of external documentation resources describing the tag.
        pub external_documentation_url: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiTagDescriptionArgs,
    ) -> ApiTagDescriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_tag_id_binding = args.api_tag_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let external_documentation_description_binding = args
            .external_documentation_description
            .get_output(context);
        let external_documentation_url_binding = args
            .external_documentation_url
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiTagDescription:ApiTagDescription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiTagId".into(),
                    value: &api_tag_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalDocumentationDescription".into(),
                    value: &external_documentation_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalDocumentationUrl".into(),
                    value: &external_documentation_url_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiTagDescriptionResult {
            api_tag_id: o.get_field("apiTagId"),
            description: o.get_field("description"),
            external_documentation_description: o
                .get_field("externalDocumentationDescription"),
            external_documentation_url: o.get_field("externalDocumentationUrl"),
        }
    }
}
