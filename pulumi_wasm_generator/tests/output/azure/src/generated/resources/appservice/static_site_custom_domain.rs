/// ## Example Usage
///
/// ### CNAME validation
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
///     let exampleCNameRecord = c_name_record::create(
///         "exampleCNameRecord",
///         CNameRecordArgs::builder()
///             .name("my-domain")
///             .record("${exampleStaticSite.defaultHostName}")
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("contoso.com")
///             .build_struct(),
///     );
///     let exampleStaticSite = static_site::create(
///         "exampleStaticSite",
///         StaticSiteArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStaticSiteCustomDomain = static_site_custom_domain::create(
///         "exampleStaticSiteCustomDomain",
///         StaticSiteCustomDomainArgs::builder()
///             .domain_name("${exampleCNameRecord.name}.${exampleCNameRecord.zoneName}")
///             .static_site_id("${exampleStaticSite.id}")
///             .validation_type("cname-delegation")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### TXT validation
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
///     let exampleStaticSite = static_site::create(
///         "exampleStaticSite",
///         StaticSiteArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStaticSiteCustomDomain = static_site_custom_domain::create(
///         "exampleStaticSiteCustomDomain",
///         StaticSiteCustomDomainArgs::builder()
///             .domain_name("my-domain.contoso.com")
///             .static_site_id("${exampleStaticSite.id}")
///             .validation_type("dns-txt-token")
///             .build_struct(),
///     );
///     let exampleTxtRecord = txt_record::create(
///         "exampleTxtRecord",
///         TxtRecordArgs::builder()
///             .name("_dnsauth.my-domain")
///             .records(
///                 vec![
///                     TxtRecordRecord::builder()
///                     .value("${exampleStaticSiteCustomDomain.validationToken}")
///                     .build_struct(),
///                 ],
///             )
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("contoso.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Static Site Custom Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/staticSiteCustomDomain:StaticSiteCustomDomain example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Web/staticSites/my-static-site1/customDomains/name.contoso.com
/// ```
///
pub mod static_site_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticSiteCustomDomainArgs {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub static_site_id: pulumi_wasm_rust::Output<String>,
        /// One of `cname-delegation` or `dns-txt-token`. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into, default)]
        pub validation_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StaticSiteCustomDomainResult {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub static_site_id: pulumi_wasm_rust::Output<String>,
        /// Token to be used with `dns-txt-token` validation.
        pub validation_token: pulumi_wasm_rust::Output<String>,
        /// One of `cname-delegation` or `dns-txt-token`. Changing this forces a new Static Site Custom Domain to be created.
        pub validation_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: StaticSiteCustomDomainArgs,
    ) -> StaticSiteCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let static_site_id_binding = args.static_site_id.get_inner();
        let validation_type_binding = args.validation_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/staticSiteCustomDomain:StaticSiteCustomDomain"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "staticSiteId".into(),
                    value: &static_site_id_binding,
                },
                register_interface::ObjectField {
                    name: "validationType".into(),
                    value: &validation_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "staticSiteId".into(),
                },
                register_interface::ResultField {
                    name: "validationToken".into(),
                },
                register_interface::ResultField {
                    name: "validationType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StaticSiteCustomDomainResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            static_site_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticSiteId").unwrap(),
            ),
            validation_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationToken").unwrap(),
            ),
            validation_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationType").unwrap(),
            ),
        }
    }
}