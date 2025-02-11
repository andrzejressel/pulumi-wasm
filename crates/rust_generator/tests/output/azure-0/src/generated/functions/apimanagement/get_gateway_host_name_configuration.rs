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
        context: &pulumi_gestalt_rust::Context,
        args: GetGatewayHostNameConfigurationArgs,
    ) -> GetGatewayHostNameConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let gateway_name_binding = args.gateway_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:apimanagement/getGatewayHostNameConfiguration:getGatewayHostNameConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGatewayHostNameConfigurationResult {
            api_management_id: o.get_field("apiManagementId"),
            certificate_id: o.get_field("certificateId"),
            gateway_name: o.get_field("gatewayName"),
            host_name: o.get_field("hostName"),
            http2_enabled: o.get_field("http2Enabled"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            request_client_certificate_enabled: o
                .get_field("requestClientCertificateEnabled"),
            tls10_enabled: o.get_field("tls10Enabled"),
            tls11_enabled: o.get_field("tls11Enabled"),
        }
    }
}
