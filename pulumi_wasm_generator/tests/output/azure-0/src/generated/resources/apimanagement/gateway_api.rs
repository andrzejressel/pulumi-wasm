/// Manages a API Management Gateway API.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGatewayApi:
///     type: azure:apimanagement:GatewayApi
///     name: example
///     properties:
///       gatewayId: ${exampleGetGateway.id}
///       apiId: ${exampleGetApi.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getService
///       arguments:
///         name: example-api
///         resourceGroupName: example-resources
///   exampleGetApi:
///     fn::invoke:
///       function: azure:apimanagement:getApi
///       arguments:
///         name: search-api
///         apiManagementName: ${example.name}
///         resourceGroupName: ${example.resourceGroupName}
///         revision: '2'
///   exampleGetGateway:
///     fn::invoke:
///       function: azure:apimanagement:getGateway
///       arguments:
///         name: example-gateway
///         apiManagementId: ${example.id}
/// ```
///
/// ## Import
///
/// API Management Gateway APIs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/gatewayApi:GatewayApi example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.ApiManagement/service/service1/gateways/gateway1/apis/api1
/// ```
///
pub mod gateway_api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayApiArgs {
        /// The Identifier of the API Management API within the API Management Service. Changing this forces a new API Management Gateway API to be created.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the API Management Gateway. Changing this forces a new API Management Gateway API to be created.
        #[builder(into)]
        pub gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GatewayApiResult {
        /// The Identifier of the API Management API within the API Management Service. Changing this forces a new API Management Gateway API to be created.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The Identifier for the API Management Gateway. Changing this forces a new API Management Gateway API to be created.
        pub gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GatewayApiArgs) -> GatewayApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let gateway_id_binding = args.gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/gatewayApi:GatewayApi".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayId".into(),
                    value: &gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GatewayApiResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayId").unwrap(),
            ),
        }
    }
}
