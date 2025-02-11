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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudCertificateArgs {
        /// The content of uploaded certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub certificate_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the private key should be excluded from the Key Vault Certificate. Changing this forces a new resource to be created. Defaults to `false`.
        #[builder(into, default)]
        pub exclude_private_key: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Key Vault Certificate resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_vault_certificate_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Spring Cloud Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudCertificateResult {
        /// The content of uploaded certificate. Changing this forces a new resource to be created.
        pub certificate_content: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether the private key should be excluded from the Key Vault Certificate. Changing this forces a new resource to be created. Defaults to `false`.
        pub exclude_private_key: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the Key Vault Certificate resource. Changing this forces a new resource to be created.
        pub key_vault_certificate_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Spring Cloud Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the resource group in which to create the Spring Cloud Certificate. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Spring Cloud Service resource. Changing this forces a new resource to be created.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the Spring Cloud certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudCertificateArgs,
    ) -> SpringCloudCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_content_binding = args.certificate_content.get_output(context);
        let exclude_private_key_binding = args.exclude_private_key.get_output(context);
        let key_vault_certificate_id_binding = args
            .key_vault_certificate_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudCertificate:SpringCloudCertificate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateContent".into(),
                    value: &certificate_content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludePrivateKey".into(),
                    value: &exclude_private_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultCertificateId".into(),
                    value: &key_vault_certificate_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: &service_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudCertificateResult {
            certificate_content: o.get_field("certificateContent"),
            exclude_private_key: o.get_field("excludePrivateKey"),
            key_vault_certificate_id: o.get_field("keyVaultCertificateId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_name: o.get_field("serviceName"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}
