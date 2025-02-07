/// Manages an API Management Gateway Certificate Authority.
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
///   exampleGatewayCertificateAuthority:
///     type: azure:apimanagement:GatewayCertificateAuthority
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       certificateName: ${exampleCertificate.name}
///       gatewayName: ${exampleGateway.name}
///       isTrusted: true
/// ```
///
/// ## Import
///
/// API Management Gateway Certificate Authority can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/gatewayCertificateAuthority:GatewayCertificateAuthority example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/gateways/gateway1/certificateAuthorities/cert1
/// ```
///
pub mod gateway_certificate_authority {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayCertificateAuthorityArgs {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub certificate_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub gateway_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the API Management Gateway Certificate Authority is trusted.
        #[builder(into, default)]
        pub is_trusted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GatewayCertificateAuthorityResult {
        /// The ID of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        pub certificate_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the API Management Gateway. Changing this forces a new resource to be created.
        pub gateway_name: pulumi_gestalt_rust::Output<String>,
        /// Whether the API Management Gateway Certificate Authority is trusted.
        pub is_trusted: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GatewayCertificateAuthorityArgs,
    ) -> GatewayCertificateAuthorityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args
            .api_management_id
            .get_output(context)
            .get_inner();
        let certificate_name_binding = args
            .certificate_name
            .get_output(context)
            .get_inner();
        let gateway_name_binding = args.gateway_name.get_output(context).get_inner();
        let is_trusted_binding = args.is_trusted.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/gatewayCertificateAuthority:GatewayCertificateAuthority"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayName".into(),
                    value: &gateway_name_binding,
                },
                register_interface::ObjectField {
                    name: "isTrusted".into(),
                    value: &is_trusted_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GatewayCertificateAuthorityResult {
            api_management_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementId"),
            ),
            certificate_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateName"),
            ),
            gateway_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayName"),
            ),
            is_trusted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isTrusted"),
            ),
        }
    }
}
