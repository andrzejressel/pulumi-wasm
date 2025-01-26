/// Manages a API Management Custom Domain.
///
/// ## Disclaimers
///
/// > **Note:** It's possible to define Custom Domains both within the `azure.apimanagement.Service` resource via the `hostname_configurations` block and by using this resource. However it's not possible to use both methods to manage Custom Domains within an API Management Service, since there will be conflicts.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleService:
///     type: azure:apimanagement:Service
///     name: example
///     properties:
///       name: example-apim
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       publisherName: pub1
///       publisherEmail: pub1@email.com
///       skuName: Developer_1
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: example-certificate
///       keyVaultId: ${example.id}
///       certificatePolicy:
///         issuerParameters:
///           name: Self
///         keyProperties:
///           exportable: true
///           keySize: 2048
///           keyType: RSA
///           reuseKey: true
///         lifetimeActions:
///           - action:
///               actionType: AutoRenew
///             trigger:
///               daysBeforeExpiry: 30
///         secretProperties:
///           contentType: application/x-pkcs12
///         x509CertificateProperties:
///           keyUsages:
///             - cRLSign
///             - dataEncipherment
///             - digitalSignature
///             - keyAgreement
///             - keyCertSign
///             - keyEncipherment
///           subject: CN=api.example.com
///           validityInMonths: 12
///           subjectAlternativeNames:
///             dnsNames:
///               - api.example.com
///               - portal.example.com
///   exampleCustomDomain:
///     type: azure:apimanagement:CustomDomain
///     name: example
///     properties:
///       apiManagementId: ${exampleService.id}
///       gateways:
///         - hostName: api.example.com
///           keyVaultId: ${exampleCertificate.versionlessSecretId}
///       developerPortals:
///         - hostName: portal.example.com
///           keyVaultId: ${exampleCertificate.versionlessSecretId}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:keyvault:getKeyVault
///       arguments:
///         name: mykeyvault
///         resourceGroupName: some-resource-group
/// ```
///
/// ## Import
///
/// API Management Custom Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/customDomain:CustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/customDomains/default
/// ```
///
pub mod custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDomainArgs {
        /// The ID of the API Management service for which to configure Custom Domains. Changing this forces a new API Management Custom Domain resource to be created.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// One or more `developer_portal` blocks as defined below.
        #[builder(into, default)]
        pub developer_portals: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::CustomDomainDeveloperPortal>>,
        >,
        /// One or more `gateway` blocks as defined below.
        #[builder(into, default)]
        pub gateways: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::CustomDomainGateway>>,
        >,
        /// One or more `management` blocks as defined below.
        #[builder(into, default)]
        pub managements: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::CustomDomainManagement>>,
        >,
        /// One or more `portal` blocks as defined below.
        #[builder(into, default)]
        pub portals: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::CustomDomainPortal>>,
        >,
        /// One or more `scm` blocks as defined below.
        #[builder(into, default)]
        pub scms: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apimanagement::CustomDomainScm>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomDomainResult {
        /// The ID of the API Management service for which to configure Custom Domains. Changing this forces a new API Management Custom Domain resource to be created.
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// One or more `developer_portal` blocks as defined below.
        pub developer_portals: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::CustomDomainDeveloperPortal>>,
        >,
        /// One or more `gateway` blocks as defined below.
        pub gateways: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::CustomDomainGateway>>,
        >,
        /// One or more `management` blocks as defined below.
        pub managements: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::CustomDomainManagement>>,
        >,
        /// One or more `portal` blocks as defined below.
        pub portals: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::CustomDomainPortal>>,
        >,
        /// One or more `scm` blocks as defined below.
        pub scms: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apimanagement::CustomDomainScm>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomDomainArgs,
    ) -> CustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args
            .api_management_id
            .get_output(context)
            .get_inner();
        let developer_portals_binding = args
            .developer_portals
            .get_output(context)
            .get_inner();
        let gateways_binding = args.gateways.get_output(context).get_inner();
        let managements_binding = args.managements.get_output(context).get_inner();
        let portals_binding = args.portals.get_output(context).get_inner();
        let scms_binding = args.scms.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/customDomain:CustomDomain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "developerPortals".into(),
                    value: &developer_portals_binding,
                },
                register_interface::ObjectField {
                    name: "gateways".into(),
                    value: &gateways_binding,
                },
                register_interface::ObjectField {
                    name: "managements".into(),
                    value: &managements_binding,
                },
                register_interface::ObjectField {
                    name: "portals".into(),
                    value: &portals_binding,
                },
                register_interface::ObjectField {
                    name: "scms".into(),
                    value: &scms_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomDomainResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementId"),
            ),
            developer_portals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("developerPortals"),
            ),
            gateways: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gateways"),
            ),
            managements: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managements"),
            ),
            portals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("portals"),
            ),
            scms: pulumi_wasm_rust::__private::into_domain(o.extract_field("scms")),
        }
    }
}
