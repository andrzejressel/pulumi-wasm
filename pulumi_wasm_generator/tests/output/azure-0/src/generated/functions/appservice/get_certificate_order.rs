pub mod get_certificate_order {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateOrderArgs {
        /// The name of the App Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the App Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateOrderResult {
        /// Reasons why App Service Certificate is not renewable at the current moment.
        pub app_service_certificate_not_renewable_reasons: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// true if the certificate should be automatically renewed when it expires; otherwise, false.
        pub auto_renew: pulumi_wasm_rust::Output<bool>,
        /// State of the Key Vault secret. A `certificates` block as defined below.
        pub certificates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetCertificateOrderCertificate>,
        >,
        /// Last CSR that was created for this order.
        pub csr: pulumi_wasm_rust::Output<String>,
        /// The Distinguished Name for the App Service Certificate Order.
        pub distinguished_name: pulumi_wasm_rust::Output<String>,
        /// Domain verification token.
        pub domain_verification_token: pulumi_wasm_rust::Output<String>,
        /// Certificate expiration time.
        pub expiration_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint intermediate certificate.
        pub intermediate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Whether the private key is external or not.
        pub is_private_key_external: pulumi_wasm_rust::Output<bool>,
        /// Certificate key size.
        pub key_size: pulumi_wasm_rust::Output<i32>,
        /// The Azure location where the App Service exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Certificate product type, such as `Standard` or `WildCard`.
        pub product_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint for root certificate.
        pub root_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint for signed certificate.
        pub signed_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Current order status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Duration in years (must be between 1 and 3).
        pub validity_in_years: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCertificateOrderArgs,
    ) -> GetCertificateOrderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getCertificateOrder:getCertificateOrder".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateOrderResult {
            app_service_certificate_not_renewable_reasons: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServiceCertificateNotRenewableReasons"),
            ),
            auto_renew: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoRenew"),
            ),
            certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            csr: pulumi_wasm_rust::__private::into_domain(o.extract_field("csr")),
            distinguished_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("distinguishedName"),
            ),
            domain_verification_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainVerificationToken"),
            ),
            expiration_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationTime"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            intermediate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("intermediateThumbprint"),
            ),
            is_private_key_external: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isPrivateKeyExternal"),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            product_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("productType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            root_thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootThumbprint"),
            ),
            signed_certificate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signedCertificateThumbprint"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            validity_in_years: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validityInYears"),
            ),
        }
    }
}
