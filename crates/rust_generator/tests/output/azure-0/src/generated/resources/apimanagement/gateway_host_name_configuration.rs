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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway_host_name_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayHostNameConfigurationArgs {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The certificate ID to be used for TLS connection establishment.
        #[builder(into)]
        pub certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The host name to use for the API Management Gateway Host Name Configuration.
        #[builder(into)]
        pub host_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether HTTP/2.0 is supported. Defaults to `true`.
        #[builder(into, default)]
        pub http2_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the API Management Gateway Host Name Configuration. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the API Management Gateway requests a client certificate.
        #[builder(into, default)]
        pub request_client_certificate_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether TLS 1.0 is supported.
        #[builder(into, default)]
        pub tls10_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether TLS 1.1 is supported.
        #[builder(into, default)]
        pub tls11_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GatewayHostNameConfigurationResult {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The certificate ID to be used for TLS connection establishment.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        pub gateway_name: pulumi_gestalt_rust::Output<String>,
        /// The host name to use for the API Management Gateway Host Name Configuration.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// Whether HTTP/2.0 is supported. Defaults to `true`.
        pub http2_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the API Management Gateway Host Name Configuration. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the API Management Gateway requests a client certificate.
        pub request_client_certificate_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether TLS 1.0 is supported.
        pub tls10_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether TLS 1.1 is supported.
        pub tls11_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GatewayHostNameConfigurationArgs,
    ) -> GatewayHostNameConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_id_binding = args.api_management_id.get_output(context);
        let certificate_id_binding = args.certificate_id.get_output(context);
        let gateway_name_binding = args.gateway_name.get_output(context);
        let host_name_binding = args.host_name.get_output(context);
        let http2_enabled_binding = args.http2_enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let request_client_certificate_enabled_binding = args
            .request_client_certificate_enabled
            .get_output(context);
        let tls10_enabled_binding = args.tls10_enabled.get_output(context);
        let tls11_enabled_binding = args.tls11_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/gatewayHostNameConfiguration:GatewayHostNameConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementId".into(),
                    value: api_management_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateId".into(),
                    value: certificate_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayName".into(),
                    value: gateway_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostName".into(),
                    value: host_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "http2Enabled".into(),
                    value: http2_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestClientCertificateEnabled".into(),
                    value: request_client_certificate_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tls10Enabled".into(),
                    value: tls10_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tls11Enabled".into(),
                    value: tls11_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayHostNameConfigurationResult {
            api_management_id: o.get_field("apiManagementId"),
            certificate_id: o.get_field("certificateId"),
            gateway_name: o.get_field("gatewayName"),
            host_name: o.get_field("hostName"),
            http2_enabled: o.get_field("http2Enabled"),
            name: o.get_field("name"),
            request_client_certificate_enabled: o
                .get_field("requestClientCertificateEnabled"),
            tls10_enabled: o.get_field("tls10Enabled"),
            tls11_enabled: o.get_field("tls11Enabled"),
        }
    }
}
