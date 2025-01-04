pub mod get_gateway_host_name_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayHostNameConfigurationArgs {
        /// The ID of the API Management Service.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The name of the API Management Gateway.
        /// *
        #[builder(into)]
        pub gateway_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API Management Gateway Host Name Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayHostNameConfigurationResult {
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the certificate used for TLS connection establishment.
        pub certificate_id: pulumi_wasm_rust::Output<String>,
        pub gateway_name: pulumi_wasm_rust::Output<String>,
        /// The host name used for the API Management Gateway Host Name Configuration.
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// Whether HTTP/2.0 is supported.
        pub http2_enabled: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the API Management Gateway requests a client certificate.
        pub request_client_certificate_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether TLS 1.0 is supported.
        pub tls10_enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether TLS 1.1 is supported.
        pub tls11_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetGatewayHostNameConfigurationArgs,
    ) -> GetGatewayHostNameConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args.api_management_id.get_inner();
        let gateway_name_binding = args.gateway_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getGatewayHostNameConfiguration:getGatewayHostNameConfiguration"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGatewayHostNameConfigurationResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
