/// Manages an API Management API Policy
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleApiPolicy:
///     type: azure:apimanagement:ApiPolicy
///     name: example
///     properties:
///       apiName: ${example.name}
///       apiManagementName: ${example.apiManagementName}
///       resourceGroupName: ${example.resourceGroupName}
///       xmlContent: |
///         <policies>
///           <inbound>
///             <find-and-replace from="xyz" to="abc" />
///           </inbound>
///         </policies>
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getApi
///       arguments:
///         name: my-api
///         apiManagementName: example-apim
///         resourceGroupName: search-service
///         revision: '2'
/// ```
///
/// ## Import
///
/// API Management API Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/apiPolicy:ApiPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/apis/exampleId
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiPolicyArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The XML Content for this Policy as a string.
        #[builder(into, default)]
        pub xml_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A link to a Policy XML Document, which must be publicly available.
        #[builder(into, default)]
        pub xml_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiPolicyResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        pub api_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The XML Content for this Policy as a string.
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
        args: ApiPolicyArgs,
    ) -> ApiPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let api_name_binding = args.api_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let xml_content_binding = args.xml_content.get_output(context);
        let xml_link_binding = args.xml_link.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/apiPolicy:ApiPolicy".into(),
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
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
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
        ApiPolicyResult {
            api_management_name: o.get_field("apiManagementName"),
            api_name: o.get_field("apiName"),
            resource_group_name: o.get_field("resourceGroupName"),
            xml_content: o.get_field("xmlContent"),
            xml_link: o.get_field("xmlLink"),
        }
    }
}
