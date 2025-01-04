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
pub mod api_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiPolicyArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The XML Content for this Policy as a string.
        #[builder(into, default)]
        pub xml_content: pulumi_wasm_rust::Output<Option<String>>,
        /// A link to a Policy XML Document, which must be publicly available.
        #[builder(into, default)]
        pub xml_link: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiPolicyResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the API Management API within the API Management Service. Changing this forces a new resource to be created.
        pub api_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The XML Content for this Policy as a string.
        pub xml_content: pulumi_wasm_rust::Output<String>,
        /// A link to a Policy XML Document, which must be publicly available.
        pub xml_link: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiPolicyArgs) -> ApiPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let api_name_binding = args.api_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let xml_content_binding = args.xml_content.get_inner();
        let xml_link_binding = args.xml_link.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/apiPolicy:ApiPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "apiName".into(),
                    value: &api_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "xmlContent".into(),
                    value: &xml_content_binding,
                },
                register_interface::ObjectField {
                    name: "xmlLink".into(),
                    value: &xml_link_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "apiName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "xmlContent".into(),
                },
                register_interface::ResultField {
                    name: "xmlLink".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiPolicyResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            api_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            xml_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xmlContent").unwrap(),
            ),
            xml_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xmlLink").unwrap(),
            ),
        }
    }
}
