/// Manages an App Service certificate.
///
/// ## Example Usage
///
/// This example provisions an App Service Certificate from a Local File.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleCertificate:
///     type: azure:appservice:Certificate
///     name: example
///     properties:
///       name: example-cert
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       pfxBlob:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.pfx
///           return: result
///       password: password123!
/// ```
///
/// ## Import
///
/// App Service Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/certificates/certificate1
/// ```
///
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The ID of the associated App Service plan. Must be specified when the certificate is used inside an App Service Environment hosted App Service or with Premium App Service plans. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub app_service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub key_vault_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Key Vault secret. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        #[builder(into, default)]
        pub key_vault_secret_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password to access the certificate's private key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The base64-encoded contents of the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        #[builder(into, default)]
        pub pfx_blob: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The resource group must be the same as that which the app service plan is defined in - otherwise the certificate will not show as available for the app services.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The ID of the associated App Service plan. Must be specified when the certificate is used inside an App Service Environment hosted App Service or with Premium App Service plans. Changing this forces a new resource to be created.
        pub app_service_plan_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The expiration date for the certificate.
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The friendly name of the certificate.
        pub friendly_name: pulumi_wasm_rust::Output<String>,
        /// List of host names the certificate applies to.
        pub host_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ID of the App Service Environment where the certificate is in use.
        pub hosting_environment_profile_id: pulumi_wasm_rust::Output<String>,
        /// The issue date for the certificate.
        pub issue_date: pulumi_wasm_rust::Output<String>,
        /// The name of the certificate issuer.
        pub issuer: pulumi_wasm_rust::Output<String>,
        pub key_vault_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Key Vault secret. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        pub key_vault_secret_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password to access the certificate's private key. Changing this forces a new resource to be created.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The base64-encoded contents of the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Exactly one of `key_vault_secret_id` or `pfx_blob` must be specified.
        pub pfx_blob: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The resource group must be the same as that which the app service plan is defined in - otherwise the certificate will not show as available for the app services.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The subject name of the certificate.
        pub subject_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The thumbprint for the certificate.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_plan_id_binding = args.app_service_plan_id.get_inner();
        let key_vault_id_binding = args.key_vault_id.get_inner();
        let key_vault_secret_id_binding = args.key_vault_secret_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let pfx_blob_binding = args.pfx_blob.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServicePlanId".into(),
                    value: &app_service_plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultSecretId".into(),
                    value: &key_vault_secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "pfxBlob".into(),
                    value: &pfx_blob_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServicePlanId".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "hostNames".into(),
                },
                register_interface::ResultField {
                    name: "hostingEnvironmentProfileId".into(),
                },
                register_interface::ResultField {
                    name: "issueDate".into(),
                },
                register_interface::ResultField {
                    name: "issuer".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "pfxBlob".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subjectName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            app_service_plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServicePlanId").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            host_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostNames").unwrap(),
            ),
            hosting_environment_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostingEnvironmentProfileId").unwrap(),
            ),
            issue_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issueDate").unwrap(),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuer").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            key_vault_secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            pfx_blob: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pfxBlob").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subject_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
        }
    }
}
