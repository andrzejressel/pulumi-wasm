/// Provides a resource to customize the pages your end users will see
/// when trying to reach applications behind Cloudflare Access.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_custom_page::create(
///         "example",
///         AccessCustomPageArgs::builder()
///             .custom_html("<html><body><h1>Forbidden</h1></body></html>")
///             .name("example")
///             .type_("forbidden")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_custom_page {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessCustomPageArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of apps to display on the custom page.
        #[builder(into, default)]
        pub app_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Custom HTML to display on the custom page.
        #[builder(into, default)]
        pub custom_html: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Custom Page configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessCustomPageResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of apps to display on the custom page.
        pub app_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Custom HTML to display on the custom page.
        pub custom_html: pulumi_gestalt_rust::Output<Option<String>>,
        /// Friendly name of the Access Custom Page configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessCustomPageArgs,
    ) -> AccessCustomPageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let app_count_binding = args.app_count.get_output(context);
        let custom_html_binding = args.custom_html.get_output(context);
        let name_binding = args.name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessCustomPage:AccessCustomPage".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appCount".into(),
                    value: app_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customHtml".into(),
                    value: custom_html_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessCustomPageResult {
            account_id: o.get_field("accountId"),
            app_count: o.get_field("appCount"),
            custom_html: o.get_field("customHtml"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
