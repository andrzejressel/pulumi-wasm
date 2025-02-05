/// Manages an Azure SignalR Custom Certificate.
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
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Custom Certificate for a SignalR service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:signalr/serviceCustomCertificate:ServiceCustomCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/signalR/signalr1/customCertificates/cert1
/// ```
///
pub mod service_custom_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceCustomCertificateArgs {
        /// The certificate id of the SignalR Custom Certificate service. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Self assigned certificate is not supported and the provisioning status will fail.
        #[builder(into)]
        pub custom_certificate_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the SignalR Custom Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The SignalR ID of the SignalR Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Custom Certificate is only available for SignalR Premium tier. Please enable managed identity in the corresponding SignalR Service and give the managed identity access to the key vault, the required permission is Get Certificate and Secret.
        #[builder(into)]
        pub signalr_service_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceCustomCertificateResult {
        /// The certificate version of the SignalR Custom Certificate service.
        pub certificate_version: pulumi_wasm_rust::Output<String>,
        /// The certificate id of the SignalR Custom Certificate service. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Self assigned certificate is not supported and the provisioning status will fail.
        pub custom_certificate_id: pulumi_wasm_rust::Output<String>,
        /// The name of the SignalR Custom Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The SignalR ID of the SignalR Custom Certificate. Changing this forces a new resource to be created.
        ///
        /// > **Note:** Custom Certificate is only available for SignalR Premium tier. Please enable managed identity in the corresponding SignalR Service and give the managed identity access to the key vault, the required permission is Get Certificate and Secret.
        pub signalr_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceCustomCertificateArgs,
    ) -> ServiceCustomCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_certificate_id_binding = args
            .custom_certificate_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let signalr_service_id_binding = args
            .signalr_service_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:signalr/serviceCustomCertificate:ServiceCustomCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customCertificateId".into(),
                    value: &custom_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "signalrServiceId".into(),
                    value: &signalr_service_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceCustomCertificateResult {
            certificate_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateVersion"),
            ),
            custom_certificate_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customCertificateId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            signalr_service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signalrServiceId"),
            ),
        }
    }
}
