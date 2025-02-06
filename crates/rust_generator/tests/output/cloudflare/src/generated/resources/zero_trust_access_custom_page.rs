/// Provides a resource to customize the pages your end users will see
/// when trying to reach applications behind Cloudflare Access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_access_custom_page::create(
///         "example",
///         ZeroTrustAccessCustomPageArgs::builder()
///             .custom_html("<html><body><h1>Forbidden</h1></body></html>")
///             .name("example")
///             .type_("forbidden")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
pub mod zero_trust_access_custom_page {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessCustomPageArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Number of apps to display on the custom page.
        #[builder(into, default)]
        pub app_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Custom HTML to display on the custom page.
        #[builder(into, default)]
        pub custom_html: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Custom Page configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessCustomPageResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of apps to display on the custom page.
        pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Custom HTML to display on the custom page.
        pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name of the Access Custom Page configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustAccessCustomPageArgs,
    ) -> ZeroTrustAccessCustomPageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let app_count_binding = args.app_count.get_output(context).get_inner();
        let custom_html_binding = args.custom_html.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessCustomPage:ZeroTrustAccessCustomPage"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "appCount".into(),
                    value: &app_count_binding,
                },
                register_interface::ObjectField {
                    name: "customHtml".into(),
                    value: &custom_html_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustAccessCustomPageResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            app_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appCount"),
            ),
            custom_html: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customHtml"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
