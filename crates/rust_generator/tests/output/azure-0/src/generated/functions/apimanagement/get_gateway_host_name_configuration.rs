#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_gateway_host_name_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayHostNameConfigurationArgs {
        /// The ID of the API Management Service.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway.
        /// *
        #[builder(into)]
        pub gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway Host Name Configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayHostNameConfigurationResult {
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the certificate used for TLS connection establishment.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        pub gateway_name: pulumi_gestalt_rust::Output<String>,
        /// The host name used for the API Management Gateway Host Name Configuration.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// Whether HTTP/2.0 is supported.
        pub http2_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the API Management Gateway requests a client certificate.
        pub request_client_certificate_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether TLS 1.0 is supported.
        pub tls10_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Whether TLS 1.1 is supported.
        pub tls11_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGatewayHostNameConfigurationArgs,
    ) -> GetGatewayHostNameConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding_1 = args.api_management_id.get_output(context);
        let api_management_id_binding = api_management_id_binding_1.get_inner();
        let gateway_name_binding_1 = args.gateway_name.get_output(context);
        let gateway_name_binding = gateway_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getGatewayHostNameConfiguration:getGatewayHostNameConfiguration"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGatewayHostNameConfigurationResult {
            api_management_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementId"),
            ),
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            gateway_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayName"),
            ),
            host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostName"),
            ),
            http2_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("http2Enabled"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            request_client_certificate_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestClientCertificateEnabled"),
            ),
            tls10_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tls10Enabled"),
            ),
            tls11_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tls11Enabled"),
            ),
        }
    }
}
