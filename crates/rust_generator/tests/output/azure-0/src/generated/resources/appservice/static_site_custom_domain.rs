/// ## Example Usage
///
/// ### CNAME validation
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod static_site_custom_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticSiteCustomDomainArgs {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into)]
        pub static_site_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One of `cname-delegation` or `dns-txt-token`. Changing this forces a new Static Site Custom Domain to be created.
        #[builder(into, default)]
        pub validation_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StaticSiteCustomDomainResult {
        /// The Domain Name which should be associated with this Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Static Site. Changing this forces a new Static Site Custom Domain to be created.
        pub static_site_id: pulumi_gestalt_rust::Output<String>,
        /// Token to be used with `dns-txt-token` validation.
        pub validation_token: pulumi_gestalt_rust::Output<String>,
        /// One of `cname-delegation` or `dns-txt-token`. Changing this forces a new Static Site Custom Domain to be created.
        pub validation_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StaticSiteCustomDomainArgs,
    ) -> StaticSiteCustomDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let static_site_id_binding = args.static_site_id.get_output(context);
        let validation_type_binding = args.validation_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/staticSiteCustomDomain:StaticSiteCustomDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticSiteId".into(),
                    value: static_site_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validationType".into(),
                    value: validation_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StaticSiteCustomDomainResult {
            domain_name: o.get_field("domainName"),
            static_site_id: o.get_field("staticSiteId"),
            validation_token: o.get_field("validationToken"),
            validation_type: o.get_field("validationType"),
        }
    }
}
