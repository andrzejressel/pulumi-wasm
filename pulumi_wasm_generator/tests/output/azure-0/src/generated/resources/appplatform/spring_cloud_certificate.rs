/// Manages an Azure Spring Cloud Certificate.
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
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: keyvaultcertexample
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           secretPermissions:
///             - Set
///           certificatePermissions:
///             - Create
///             - Delete
///             - Get
///             - Update
///         - tenantId: ${current.tenantId}
///           objectId: ${example.objectId}
///           secretPermissions:
///             - Get
///             - List
///           certificatePermissions:
///             - Get
///             - List
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: cert-example
///       keyVaultId: ${exampleKeyVault.id}
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
///           subject: CN=contoso.com
///           validityInMonths: 12
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example-springcloud
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///   exampleSpringCloudCertificate:
///     type: azure:appplatform:SpringCloudCertificate
///     name: example
///     properties:
///       name: example-scc
///       resourceGroupName: ${exampleSpringCloudService.resourceGroupName}
///       serviceName: ${exampleSpringCloudService.name}
///       keyVaultCertificateId: ${exampleCertificate.id}
///       excludePrivateKey: true
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Azure Spring Cloud Resource Provider
/// ```
///
/// ## Import
///
/// Spring Cloud Certificate can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudCertificate:SpringCloudCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AppPlatform/spring/spring1/certificates/cert1
/// ```
///
pub mod spring_cloud_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudCertificateArgs {
        /// The content of uploaded certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub certificate_content: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the private key should be excluded from the Key Vault Certificate. Changing this forces a new resource to be created. Defaults to `false`.
        #[builder(into, default)]
        pub exclude_private_key: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Key Vault Certificate resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_vault_certificate_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Spring Cloud Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudCertificateResult {
        /// The content of uploaded certificate. Changing this forces a new resource to be created.
        pub certificate_content: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the private key should be excluded from the Key Vault Certificate. Changing this forces a new resource to be created. Defaults to `false`.
        pub exclude_private_key: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the ID of the Key Vault Certificate resource. Changing this forces a new resource to be created.
        pub key_vault_certificate_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Certificate. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the Spring Cloud certificate.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpringCloudCertificateArgs,
    ) -> SpringCloudCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_content_binding = args
            .certificate_content
            .get_output(context)
            .get_inner();
        let exclude_private_key_binding = args
            .exclude_private_key
            .get_output(context)
            .get_inner();
        let key_vault_certificate_id_binding = args
            .key_vault_certificate_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudCertificate:SpringCloudCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateContent".into(),
                    value: &certificate_content_binding,
                },
                register_interface::ObjectField {
                    name: "excludePrivateKey".into(),
                    value: &exclude_private_key_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultCertificateId".into(),
                    value: &key_vault_certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudCertificateResult {
            certificate_content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateContent"),
            ),
            exclude_private_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludePrivateKey"),
            ),
            key_vault_certificate_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultCertificateId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
        }
    }
}
