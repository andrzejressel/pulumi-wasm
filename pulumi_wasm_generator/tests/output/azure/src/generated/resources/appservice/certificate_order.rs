/// Manages an App Service Certificate Order.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod certificate_order {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateOrderArgs {
        /// true if the certificate should be automatically renewed when it expires; otherwise, false. Defaults to `true`.
        #[builder(into, default)]
        pub auto_renew: pulumi_wasm_rust::Output<Option<bool>>,
        /// Last CSR that was created for this order.
        #[builder(into, default)]
        pub csr: pulumi_wasm_rust::Output<Option<String>>,
        /// The Distinguished Name for the App Service Certificate Order.
        ///
        /// > **NOTE:** Either `csr` or `distinguished_name` must be set - but not both.
        #[builder(into, default)]
        pub distinguished_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate key size. Defaults to `2048`.
        #[builder(into, default)]
        pub key_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Currently the only valid value is `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate product type, such as `Standard` or `WildCard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub product_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// (Optional) A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration in years (must be between `1` and `3`). Defaults to `1`.
        #[builder(into, default)]
        pub validity_in_years: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct CertificateOrderResult {
        /// Reasons why App Service Certificate is not renewable at the current moment.
        pub app_service_certificate_not_renewable_reasons: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// true if the certificate should be automatically renewed when it expires; otherwise, false. Defaults to `true`.
        pub auto_renew: pulumi_wasm_rust::Output<Option<bool>>,
        /// State of the Key Vault secret. A `certificates` block as defined below.
        pub certificates: pulumi_wasm_rust::Output<
            Vec<super::super::types::appservice::CertificateOrderCertificate>,
        >,
        /// Last CSR that was created for this order.
        pub csr: pulumi_wasm_rust::Output<String>,
        /// The Distinguished Name for the App Service Certificate Order.
        ///
        /// > **NOTE:** Either `csr` or `distinguished_name` must be set - but not both.
        pub distinguished_name: pulumi_wasm_rust::Output<String>,
        /// Domain verification token.
        pub domain_verification_token: pulumi_wasm_rust::Output<String>,
        /// Certificate expiration time.
        pub expiration_time: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint intermediate certificate.
        pub intermediate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Whether the private key is external or not.
        pub is_private_key_external: pulumi_wasm_rust::Output<bool>,
        /// Certificate key size. Defaults to `2048`.
        pub key_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Currently the only valid value is `global`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the certificate. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Certificate product type, such as `Standard` or `WildCard`. Defaults to `Standard`.
        pub product_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the certificate. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint for root certificate.
        pub root_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Certificate thumbprint for signed certificate.
        pub signed_certificate_thumbprint: pulumi_wasm_rust::Output<String>,
        /// Current order status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// (Optional) A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Duration in years (must be between `1` and `3`). Defaults to `1`.
        pub validity_in_years: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateOrderArgs) -> CertificateOrderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_renew_binding = args.auto_renew.get_inner();
        let csr_binding = args.csr.get_inner();
        let distinguished_name_binding = args.distinguished_name.get_inner();
        let key_size_binding = args.key_size.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let product_type_binding = args.product_type.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let validity_in_years_binding = args.validity_in_years.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/certificateOrder:CertificateOrder".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "appServiceCertificateNotRenewableReasons".into(),
                },
                register_interface::ResultField {
                    name: "autoRenew".into(),
                },
                register_interface::ResultField {
                    name: "certificates".into(),
                },
                register_interface::ResultField {
                    name: "csr".into(),
                },
                register_interface::ResultField {
                    name: "distinguishedName".into(),
                },
                register_interface::ResultField {
                    name: "domainVerificationToken".into(),
                },
                register_interface::ResultField {
                    name: "expirationTime".into(),
                },
                register_interface::ResultField {
                    name: "intermediateThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "isPrivateKeyExternal".into(),
                },
                register_interface::ResultField {
                    name: "keySize".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "productType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rootThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "signedCertificateThumbprint".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "validityInYears".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateOrderResult {
            app_service_certificate_not_renewable_reasons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appServiceCertificateNotRenewableReasons").unwrap(),
            ),
            auto_renew: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRenew").unwrap(),
            ),
            certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificates").unwrap(),
            ),
            csr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("csr").unwrap(),
            ),
            distinguished_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("distinguishedName").unwrap(),
            ),
            domain_verification_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainVerificationToken").unwrap(),
            ),
            expiration_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationTime").unwrap(),
            ),
            intermediate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("intermediateThumbprint").unwrap(),
            ),
            is_private_key_external: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isPrivateKeyExternal").unwrap(),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySize").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            product_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            root_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootThumbprint").unwrap(),
            ),
            signed_certificate_thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signedCertificateThumbprint").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            validity_in_years: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validityInYears").unwrap(),
            ),
        }
    }
}