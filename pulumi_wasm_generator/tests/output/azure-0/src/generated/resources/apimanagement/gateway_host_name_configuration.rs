/// Manages an API Management Gateway Host Name Configuration.
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
///       skuName: Consumption_0
///   exampleGateway:
///     type: azure:apimanagement:Gateway
///     name: example
///     properties:
///       name: example-gateway
///       apiManagementId: ${exampleService.id}
///       description: Example API Management gateway
///       locationData:
///         name: example name
///         city: example city
///         district: example district
///         region: example region
///   exampleCertificate:
///     type: azure:apimanagement:Certificate
///     name: example
///     properties:
///       name: example-cert
///       apiManagementName: ${exampleService.name}
///       resourceGroupName: ${example.name}
///       data:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: example.pfx
///           return: result
///   exampleGatewayHostNameConfiguration:
///     type: azure:apimanagement:GatewayHostNameConfiguration
///     name: example
///     properties:
///       name: example-host-name-configuration
///       apiManagementId: ${exampleService.id}
///       gatewayName: ${exampleGateway.name}
///       certificateId: ${exampleCertificate.id}
///       hostName: example-host-name
///       requestClientCertificateEnabled: true
///       http2Enabled: true
///       tls10Enabled: true
///       tls11Enabled: false
/// ```
///
/// ## Import
///
/// API Management Gateway Host Name Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/gatewayHostNameConfiguration:GatewayHostNameConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/gateways/gateway1/hostnameConfigurations/hc1
/// ```
///
pub mod gateway_host_name_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayHostNameConfigurationArgs {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The certificate ID to be used for TLS connection establishment.
        #[builder(into)]
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gateway_name: pulumi_wasm_rust::Output<String>,
        /// The host name to use for the API Management Gateway Host Name Configuration.
        #[builder(into)]
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// Whether HTTP/2.0 is supported. Defaults to `true`.
        #[builder(into, default)]
        pub http2_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the API Management Gateway Host Name Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the API Management Gateway requests a client certificate.
        #[builder(into, default)]
        pub request_client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether TLS 1.0 is supported.
        #[builder(into, default)]
        pub tls10_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether TLS 1.1 is supported.
        #[builder(into, default)]
        pub tls11_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GatewayHostNameConfigurationResult {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The certificate ID to be used for TLS connection establishment.
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        pub gateway_name: pulumi_wasm_rust::Output<String>,
        /// The host name to use for the API Management Gateway Host Name Configuration.
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// Whether HTTP/2.0 is supported. Defaults to `true`.
        pub http2_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the API Management Gateway Host Name Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the API Management Gateway requests a client certificate.
        pub request_client_certificate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether TLS 1.0 is supported.
        pub tls10_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether TLS 1.1 is supported.
        pub tls11_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: GatewayHostNameConfigurationArgs,
    ) -> GatewayHostNameConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args.api_management_id.get_inner();
        let certificate_id_binding = args.certificate_id.get_inner();
        let gateway_name_binding = args.gateway_name.get_inner();
        let host_name_binding = args.host_name.get_inner();
        let http2_enabled_binding = args.http2_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let request_client_certificate_enabled_binding = args
            .request_client_certificate_enabled
            .get_inner();
        let tls10_enabled_binding = args.tls10_enabled.get_inner();
        let tls11_enabled_binding = args.tls11_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/gatewayHostNameConfiguration:GatewayHostNameConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding,
                },
                register_interface::ObjectField {
                    name: "hostName".into(),
                    value: &host_name_binding,
                },
                register_interface::ObjectField {
                    name: "http2Enabled".into(),
                    value: &http2_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requestClientCertificateEnabled".into(),
                    value: &request_client_certificate_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tls10Enabled".into(),
                    value: &tls10_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tls11Enabled".into(),
                    value: &tls11_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementId".into(),
                },
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayName".into(),
                },
                register_interface::ResultField {
                    name: "hostName".into(),
                },
                register_interface::ResultField {
                    name: "http2Enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requestClientCertificateEnabled".into(),
                },
                register_interface::ResultField {
                    name: "tls10Enabled".into(),
                },
                register_interface::ResultField {
                    name: "tls11Enabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GatewayHostNameConfigurationResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementId").unwrap(),
            ),
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
            gateway_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayName").unwrap(),
            ),
            host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostName").unwrap(),
            ),
            http2_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("http2Enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            request_client_certificate_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestClientCertificateEnabled").unwrap(),
            ),
            tls10_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tls10Enabled").unwrap(),
            ),
            tls11_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tls11Enabled").unwrap(),
            ),
        }
    }
}
