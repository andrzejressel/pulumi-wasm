pub mod get_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// The name of the NGINX Certificate.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the NGINX Deployment that the certificate is associated with.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The path to the certificate file of the certificate.
        pub certificate_virtual_path: pulumi_wasm_rust::Output<String>,
        /// The error code of the certificate error, if any.
        pub error_code: pulumi_wasm_rust::Output<String>,
        /// The error message of the certificate error, if any.
        pub error_message: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date/time the certificate was created in Azure Key Vault.
        pub key_vault_secret_creation_date: pulumi_wasm_rust::Output<String>,
        /// The ID of the Key Vault Secret for the certificate.
        pub key_vault_secret_id: pulumi_wasm_rust::Output<String>,
        /// The version of the certificate.
        pub key_vault_secret_version: pulumi_wasm_rust::Output<String>,
        /// The path to the key file of the certificate.
        pub key_virtual_path: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
        /// The SHA-1 thumbprint of the certificate.
        pub sha1_thumbprint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCertificateArgs) -> GetCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:nginx/getCertificate:getCertificate".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateVirtualPath".into(),
                },
                register_interface::ResultField {
                    name: "errorCode".into(),
                },
                register_interface::ResultField {
                    name: "errorMessage".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretCreationDate".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretId".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretVersion".into(),
                },
                register_interface::ResultField {
                    name: "keyVirtualPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nginxDeploymentId".into(),
                },
                register_interface::ResultField {
                    name: "sha1Thumbprint".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCertificateResult {
            certificate_virtual_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateVirtualPath").unwrap(),
            ),
            error_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorCode").unwrap(),
            ),
            error_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errorMessage").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_vault_secret_creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretCreationDate").unwrap(),
            ),
            key_vault_secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretId").unwrap(),
            ),
            key_vault_secret_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretVersion").unwrap(),
            ),
            key_virtual_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVirtualPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nginx_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nginxDeploymentId").unwrap(),
            ),
            sha1_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sha1Thumbprint").unwrap(),
            ),
        }
    }
}