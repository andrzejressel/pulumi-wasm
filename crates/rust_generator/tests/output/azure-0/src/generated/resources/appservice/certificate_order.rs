/// Manages an App Service Certificate Order.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCertificateOrder = certificate_order::create(
///         "exampleCertificateOrder",
///         CertificateOrderArgs::builder()
///             .distinguished_name("CN=example.com")
///             .location("global")
///             .name("example-cert-order")
///             .product_type("Standard")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Certificate Orders can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/certificateOrder:CertificateOrder example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.CertificateRegistration/certificateOrders/certificateorder1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_order {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateOrderArgs {
        /// true if the certificate should be automatically renewed when it expires; otherwise, false. Defaults to `true`.
        #[builder(into, default)]
        pub auto_renew: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Last CSR that was created for this order.
        #[builder(into, default)]
        pub csr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Distinguished Name for the App Service Certificate Order.
        ///
        /// > **NOTE:** Either `csr` or `distinguished_name` must be set - but not both.
        #[builder(into, default)]
        pub distinguished_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Certificate key size. Defaults to `2048`.
        #[builder(into, default)]
        pub key_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Currently the only valid value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Certificate product type, such as `Standard` or `WildCard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub product_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// (Optional) A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration in years (must be between `1` and `3`). Defaults to `1`.
        #[builder(into, default)]
        pub validity_in_years: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CertificateOrderResult {
        /// Reasons why App Service Certificate is not renewable at the current moment.
        pub app_service_certificate_not_renewable_reasons: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// true if the certificate should be automatically renewed when it expires; otherwise, false. Defaults to `true`.
        pub auto_renew: pulumi_gestalt_rust::Output<Option<bool>>,
        /// State of the Key Vault secret. A `certificates` block as defined below.
        pub certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::CertificateOrderCertificate>,
        >,
        /// Last CSR that was created for this order.
        pub csr: pulumi_gestalt_rust::Output<String>,
        /// The Distinguished Name for the App Service Certificate Order.
        ///
        /// > **NOTE:** Either `csr` or `distinguished_name` must be set - but not both.
        pub distinguished_name: pulumi_gestalt_rust::Output<String>,
        /// Domain verification token.
        pub domain_verification_token: pulumi_gestalt_rust::Output<String>,
        /// Certificate expiration time.
        pub expiration_time: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint intermediate certificate.
        pub intermediate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Whether the private key is external or not.
        pub is_private_key_external: pulumi_gestalt_rust::Output<bool>,
        /// Certificate key size. Defaults to `2048`.
        pub key_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Currently the only valid value is `global`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Certificate product type, such as `Standard` or `WildCard`. Defaults to `Standard`.
        pub product_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint for root certificate.
        pub root_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Certificate thumbprint for signed certificate.
        pub signed_certificate_thumbprint: pulumi_gestalt_rust::Output<String>,
        /// Current order status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// (Optional) A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration in years (must be between `1` and `3`). Defaults to `1`.
        pub validity_in_years: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateOrderArgs,
    ) -> CertificateOrderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_renew_binding_1 = args.auto_renew.get_output(context);
        let auto_renew_binding = auto_renew_binding_1.get_inner();
        let csr_binding_1 = args.csr.get_output(context);
        let csr_binding = csr_binding_1.get_inner();
        let distinguished_name_binding_1 = args.distinguished_name.get_output(context);
        let distinguished_name_binding = distinguished_name_binding_1.get_inner();
        let key_size_binding_1 = args.key_size.get_output(context);
        let key_size_binding = key_size_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let product_type_binding_1 = args.product_type.get_output(context);
        let product_type_binding = product_type_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let validity_in_years_binding_1 = args.validity_in_years.get_output(context);
        let validity_in_years_binding = validity_in_years_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/certificateOrder:CertificateOrder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoRenew".into(),
                    value: &auto_renew_binding,
                },
                register_interface::ObjectField {
                    name: "csr".into(),
                    value: &csr_binding,
                },
                register_interface::ObjectField {
                    name: "distinguishedName".into(),
                    value: &distinguished_name_binding,
                },
                register_interface::ObjectField {
                    name: "keySize".into(),
                    value: &key_size_binding,
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
                    name: "productType".into(),
                    value: &product_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "validityInYears".into(),
                    value: &validity_in_years_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateOrderResult {
            app_service_certificate_not_renewable_reasons: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceCertificateNotRenewableReasons"),
            ),
            auto_renew: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoRenew"),
            ),
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            csr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("csr")),
            distinguished_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("distinguishedName"),
            ),
            domain_verification_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainVerificationToken"),
            ),
            expiration_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationTime"),
            ),
            intermediate_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("intermediateThumbprint"),
            ),
            is_private_key_external: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isPrivateKeyExternal"),
            ),
            key_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            product_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productType"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            root_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootThumbprint"),
            ),
            signed_certificate_thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signedCertificateThumbprint"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            validity_in_years: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validityInYears"),
            ),
        }
    }
}
