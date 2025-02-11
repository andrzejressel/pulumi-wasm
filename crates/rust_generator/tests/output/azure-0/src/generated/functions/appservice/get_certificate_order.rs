#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate_order {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateOrderArgs {
        /// The name of the App Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the App Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateOrderResult {
        /// Reasons why App Service Certificate is not renewable at the current moment.
        pub app_service_certificate_not_renewable_reasons: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// true if the certificate should be automatically renewed when it expires; otherwise, false.
        pub auto_renew: pulumi_gestalt_rust::Output<bool>,
        /// State of the Key Vault secret. A `certificates` block as defined below.
        pub certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetCertificateOrderCertificate>,
        >,
        /// Last CSR that was created for this order.
        pub csr: pulumi_gestalt_rust::Output<String>,
        /// The Distinguished Name for the App Service Certificate Order.
        pub distinguished_name: pulumi_gestalt_rust::Output<String>,
        /// Domain verification token.
        pub domain_verification_token: pulumi_gestalt_rust::Output<String>,
        /// Certificate expiration time.
        pub expiration_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint intermediate certificate.
        pub intermediate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Whether the private key is external or not.
        pub is_private_key_external: pulumi_gestalt_rust::Output<bool>,
        /// Certificate key size.
        pub key_size: pulumi_gestalt_rust::Output<i32>,
        /// The Azure location where the App Service exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Certificate product type, such as `Standard` or `WildCard`.
        pub product_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint for root certificate.
        pub root_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint for signed certificate.
        pub signed_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Current order status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Duration in years (must be between 1 and 3).
        pub validity_in_years: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateOrderArgs,
    ) -> GetCertificateOrderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getCertificateOrder:getCertificateOrder".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateOrderResult {
            app_service_certificate_not_renewable_reasons: o
                .get_field("appServiceCertificateNotRenewableReasons"),
            auto_renew: o.get_field("autoRenew"),
            certificates: o.get_field("certificates"),
            csr: o.get_field("csr"),
            distinguished_name: o.get_field("distinguishedName"),
            domain_verification_token: o.get_field("domainVerificationToken"),
            expiration_time: o.get_field("expirationTime"),
            id: o.get_field("id"),
            intermediate_thumbprint: o.get_field("intermediateThumbprint"),
            is_private_key_external: o.get_field("isPrivateKeyExternal"),
            key_size: o.get_field("keySize"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            product_type: o.get_field("productType"),
            resource_group_name: o.get_field("resourceGroupName"),
            root_thumbprint: o.get_field("rootThumbprint"),
            signed_certificate_thumbprint: o.get_field("signedCertificateThumbprint"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            validity_in_years: o.get_field("validityInYears"),
        }
    }
}
