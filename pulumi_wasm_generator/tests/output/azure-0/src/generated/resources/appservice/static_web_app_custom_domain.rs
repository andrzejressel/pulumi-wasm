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
///             .record("${exampleStaticWebApp.defaultHostName}")
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("contoso.com")
///             .build_struct(),
///     );
///     let exampleStaticWebApp = static_web_app::create(
///         "exampleStaticWebApp",
///         StaticWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStaticWebAppCustomDomain = static_web_app_custom_domain::create(
///         "exampleStaticWebAppCustomDomain",
///         StaticWebAppCustomDomainArgs::builder()
///             .domain_name("${exampleCNameRecord.name}.${exampleCNameRecord.zoneName}")
///             .static_web_app_id("${exampleStaticWebApp.id}")
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
///     let exampleStaticWebApp = static_web_app::create(
///         "exampleStaticWebApp",
///         StaticWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStaticWebAppCustomDomain = static_web_app_custom_domain::create(
///         "exampleStaticWebAppCustomDomain",
///         StaticWebAppCustomDomainArgs::builder()
///             .domain_name("my-domain.contoso.com")
///             .static_web_app_id("${exampleStaticWebApp.id}")
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
///                     .value("${exampleStaticWebAppCustomDomain.validationToken}")
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
/// $ pulumi import azure:appservice/staticWebAppCustomDomain:StaticWebAppCustomDomain example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Web/staticSites/my-static-site1/customDomains/name.contoso.com
/// ```
///
pub mod static_web_app_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticWebAppCustomDomainArgs {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub static_web_app_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub validation_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StaticWebAppCustomDomainResult {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub static_web_app_id: pulumi_wasm_rust::Output<String>,
        /// Token to be used with `dns-txt-token` validation.
        pub validation_token: pulumi_wasm_rust::Output<String>,
        pub validation_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StaticWebAppCustomDomainArgs,
    ) -> StaticWebAppCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let static_web_app_id_binding = args
            .static_web_app_id
            .get_output(context)
            .get_inner();
        let validation_type_binding = args
            .validation_type
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/staticWebAppCustomDomain:StaticWebAppCustomDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "staticWebAppId".into(),
                    value: &static_web_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "validationType".into(),
                    value: &validation_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StaticWebAppCustomDomainResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            static_web_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("staticWebAppId"),
            ),
            validation_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validationToken"),
            ),
            validation_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validationType"),
            ),
        }
    }
}
