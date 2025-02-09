/// Manages an Azure SignalR Custom Domain.
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
///     type: azure:signalr:Service
///     name: example
///     properties:
///       name: example-signalr
///       location: ${testAzurermResourceGroup.location}
///       resourceGroupName: ${testAzurermResourceGroup.name}
///       sku:
///         name: Premium_P1
///         capacity: 1
///       identity:
///         type: SystemAssigned
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: example-keyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Create
///             - Get
///             - List
///           secretPermissions:
///             - Get
///             - List
///         - tenantId: ${current.tenantId}
///           objectId: ${testAzurermSignalrService.identity[0].principalId}
///           certificatePermissions:
///             - Create
///             - Get
///             - List
///           secretPermissions:
///             - Get
///             - List
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: imported-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: certificate-to-import.pfx
///             return: result
///         password: ""
///   test:
///     type: azure:signalr:ServiceCustomCertificate
///     properties:
///       name: example-cert
///       signalrServiceId: ${exampleService.id}
///       customCertificateId: ${exampleCertificate.id}
///     options:
///       dependsOn:
///         - ${exampleAzurermKeyVaultAccessPolicy}
///   testServiceCustomDomain:
///     type: azure:signalr:ServiceCustomDomain
///     name: test
///     properties:
///       name: example-domain
///       signalrServiceId: ${testAzurermSignalrService.id}
///       domainName: tftest.com
///       signalrCustomCertificateId: ${test.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Custom Domain for a SignalR service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:signalr/serviceCustomDomain:ServiceCustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/signalR/signalr1/customDomains/customDomain1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceCustomDomainArgs {
        /// Specifies the custom domain name of the SignalR Custom Domain. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Please ensure the custom domain name is included in the Subject Alternative Names of the selected SignalR Custom Certificate.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the SignalR Custom Domain. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the SignalR Custom Certificate ID of the SignalR Custom Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub signalr_custom_certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the SignalR ID of the SignalR Custom Domain. Changing this forces a new resource to be created.
        #[builder(into)]
        pub signalr_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceCustomDomainResult {
        /// Specifies the custom domain name of the SignalR Custom Domain. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Please ensure the custom domain name is included in the Subject Alternative Names of the selected SignalR Custom Certificate.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the SignalR Custom Domain. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SignalR Custom Certificate ID of the SignalR Custom Domain. Changing this forces a new resource to be created.
        pub signalr_custom_certificate_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SignalR ID of the SignalR Custom Domain. Changing this forces a new resource to be created.
        pub signalr_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceCustomDomainArgs,
    ) -> ServiceCustomDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let signalr_custom_certificate_id_binding = args
            .signalr_custom_certificate_id
            .get_output(context);
        let signalr_service_id_binding = args.signalr_service_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:signalr/serviceCustomDomain:ServiceCustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signalrCustomCertificateId".into(),
                    value: signalr_custom_certificate_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signalrServiceId".into(),
                    value: signalr_service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceCustomDomainResult {
            domain_name: o.get_field("domainName"),
            name: o.get_field("name"),
            signalr_custom_certificate_id: o.get_field("signalrCustomCertificateId"),
            signalr_service_id: o.get_field("signalrServiceId"),
        }
    }
}
