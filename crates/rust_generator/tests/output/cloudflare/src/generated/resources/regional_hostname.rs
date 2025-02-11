/// Provides a Data Localization Suite Regional Hostname.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = record::create(
///         "example",
///         RecordArgs::builder()
///             .content("192.0.2.1")
///             .name("example.com")
///             .ttl(3600)
///             .type_("A")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let exampleRegionalHostname = regional_hostname::create(
///         "exampleRegionalHostname",
///         RegionalHostnameArgs::builder()
///             .hostname("example.com")
///             .region_key("eu")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod regional_hostname {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionalHostnameArgs {
        /// The hostname to regionalize.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
        #[builder(into)]
        pub region_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionalHostnameResult {
        /// The RFC3339 timestamp of when the hostname was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// The hostname to regionalize.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
        pub region_key: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionalHostnameArgs,
    ) -> RegionalHostnameResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hostname_binding = args.hostname.get_output(context);
        let region_key_binding = args.region_key.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/regionalHostname:RegionalHostname".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regionKey".into(),
                    value: &region_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionalHostnameResult {
            created_on: o.get_field("createdOn"),
            hostname: o.get_field("hostname"),
            region_key: o.get_field("regionKey"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
