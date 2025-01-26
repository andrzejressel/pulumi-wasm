/// Manages an Certificate within an API Management Service.
///
/// ## Example Usage
///
/// ### With Base64 Certificate)
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
///       publisherName: My Company
///       publisherEmail: company@exmaple.com
///       skuName: Developer_1
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
/// ```
///
///
/// ### With Key Vault Certificate)
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
///       publisherName: My Company
///       publisherEmail: company@terraform.io
///       skuName: Developer_1
///       identity:
///         type: SystemAssigned
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: standard
///   exampleAccessPolicy:
///     type: azure:keyvault:AccessPolicy
///     name: example
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleService.identity.tenantId}
///       objectId: ${exampleService.identity.principalId}
///       secretPermissions:
///         - Get
///       certificatePermissions:
///         - Get
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: example-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: example_cert.pfx
///             return: result
///         password: terraform
///       certificatePolicy:
///         issuerParameters:
///           name: Self
///         keyProperties:
///           exportable: true
///           keySize: 2048
///           keyType: RSA
///           reuseKey: false
///         secretProperties:
///           contentType: application/x-pkcs12
///   exampleCertificate2:
///     type: azure:apimanagement:Certificate
///     name: example
///     properties:
///       name: example-cert
///       apiManagementName: ${exampleService.name}
///       resourceGroupName: ${example.name}
///       keyVaultSecretId: ${exampleCertificate.secretId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// API Management Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/certificates/certificate1
/// ```
///
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The Name of the API Management Service where this Service should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The base-64 encoded certificate data, which must be a PFX file.
        #[builder(into, default)]
        pub data: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Client ID of the User Assigned Managed Identity to use for retrieving certificate.
        ///
        /// > **NOTE:** If not specified, will use System Assigned identity of the API Management Service.
        #[builder(into, default)]
        pub key_vault_identity_client_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the Key Vault Secret containing the SSL Certificate, which must be of the type `application/x-pkcs12`.
        ///
        /// > **NOTE:** Setting this field requires the `identity` block to be specified in API Management Service, since this identity is used to retrieve the Key Vault Certificate. Possible values are versioned or versionless secret ID. Auto-updating the Certificate from the Key Vault requires that Secret version isn't specified.
        #[builder(into, default)]
        pub key_vault_secret_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The password used for this certificate.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either `data` or `key_vault_secret_id` must be specified - but not both.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The Name of the API Management Service where this Service should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The base-64 encoded certificate data, which must be a PFX file.
        pub data: pulumi_wasm_rust::Output<Option<String>>,
        /// The Expiration Date of this Certificate, formatted as an RFC3339 string.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// The Client ID of the User Assigned Managed Identity to use for retrieving certificate.
        ///
        /// > **NOTE:** If not specified, will use System Assigned identity of the API Management Service.
        pub key_vault_identity_client_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Key Vault Secret containing the SSL Certificate, which must be of the type `application/x-pkcs12`.
        ///
        /// > **NOTE:** Setting this field requires the `identity` block to be specified in API Management Service, since this identity is used to retrieve the Key Vault Certificate. Possible values are versioned or versionless secret ID. Auto-updating the Certificate from the Key Vault requires that Secret version isn't specified.
        pub key_vault_secret_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the API Management Certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password used for this certificate.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The Name of the Resource Group where the API Management Service exists. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Either `data` or `key_vault_secret_id` must be specified - but not both.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Subject of this Certificate.
        pub subject: pulumi_wasm_rust::Output<String>,
        /// The Thumbprint of this Certificate.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let data_binding = args.data.get_output(context).get_inner();
        let key_vault_identity_client_id_binding = args
            .key_vault_identity_client_id
            .get_output(context)
            .get_inner();
        let key_vault_secret_id_binding = args
            .key_vault_secret_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultIdentityClientId".into(),
                    value: &key_vault_identity_client_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultSecretId".into(),
                    value: &key_vault_secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            data: pulumi_wasm_rust::__private::into_domain(o.extract_field("data")),
            expiration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiration"),
            ),
            key_vault_identity_client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultIdentityClientId"),
            ),
            key_vault_secret_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultSecretId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subject: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subject"),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
        }
    }
}
