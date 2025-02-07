pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// The name of the NGINX Certificate.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the NGINX Deployment that the certificate is associated with.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The path to the certificate file of the certificate.
        pub certificate_virtual_path: pulumi_gestalt_rust::Output<String>,
        /// The error code of the certificate error, if any.
        pub error_code: pulumi_gestalt_rust::Output<String>,
        /// The error message of the certificate error, if any.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date/time the certificate was created in Azure Key Vault.
        pub key_vault_secret_creation_date: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Key Vault Secret for the certificate.
        pub key_vault_secret_id: pulumi_gestalt_rust::Output<String>,
        /// The version of the certificate.
        pub key_vault_secret_version: pulumi_gestalt_rust::Output<String>,
        /// The path to the key file of the certificate.
        pub key_virtual_path: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub nginx_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// The SHA-1 thumbprint of the certificate.
        pub sha1_thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let nginx_deployment_id_binding = args
            .nginx_deployment_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:nginx/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateResult {
            certificate_virtual_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateVirtualPath"),
            ),
            error_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("errorCode"),
            ),
            error_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("errorMessage"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_vault_secret_creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultSecretCreationDate"),
            ),
            key_vault_secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultSecretId"),
            ),
            key_vault_secret_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultSecretVersion"),
            ),
            key_virtual_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVirtualPath"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nginx_deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nginxDeploymentId"),
            ),
            sha1_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sha1Thumbprint"),
            ),
        }
    }
}
