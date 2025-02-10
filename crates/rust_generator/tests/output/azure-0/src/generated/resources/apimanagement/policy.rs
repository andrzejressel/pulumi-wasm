/// Manages a API Management service Policy.
///
/// > **NOTE:** This resource will, upon creation, **overwrite any existing policy in the API Management service**, as there is no feasible way to test whether the policy has been modified from the default. Similarly, when this resource is destroyed, the API Management service will revert to its default policy.
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
///       publisherName: pub1
///       publisherEmail: pub1@email.com
///       skuName: Developer_1
///   exampleNamedValue:
///     type: azure:apimanagement:NamedValue
///     name: example
///     properties:
///       name: example-apimg
///       resourceGroupName: ${example.name}
///       apiManagementName: ${exampleService.name}
///       displayName: ExampleProperty
///       value: Example Value
///   examplePolicy:
///     type: azure:apimanagement:Policy
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       xmlContent:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: example.xml
///           return: result
/// ```
///
/// ## Import
///
/// API Management service Policys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/policy:Policy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// The ID of the API Management service. Changing this forces a new API Management service Policy to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The XML Content for this Policy as a string. To integrate frontend and backend services in Azure API Management, utilize the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, specifying the `base-url` value. Typically, this value corresponds to the `url` property defined in the `Backend` resource configuration.
        #[builder(into, default)]
        pub xml_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A link to a Policy XML Document, which must be publicly available.
        #[builder(into, default)]
        pub xml_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// The ID of the API Management service. Changing this forces a new API Management service Policy to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The XML Content for this Policy as a string. To integrate frontend and backend services in Azure API Management, utilize the [`set-backend-service`](https://learn.microsoft.com/azure/api-management/set-backend-service-policy) policy, specifying the `base-url` value. Typically, this value corresponds to the `url` property defined in the `Backend` resource configuration.
        pub xml_content: pulumi_gestalt_rust::Output<String>,
        /// A link to a Policy XML Document, which must be publicly available.
        pub xml_link: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let xml_content_binding = args.xml_content.get_output(context);
        let xml_link_binding = args.xml_link.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: api_management_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xmlContent".into(),
                    value: xml_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xmlLink".into(),
                    value: xml_link_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            api_management_id: o.get_field("apiManagementId"),
            xml_content: o.get_field("xmlContent"),
            xml_link: o.get_field("xmlLink"),
        }
    }
}
