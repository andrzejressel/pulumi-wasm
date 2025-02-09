/// Provides a resource which manages Total TLS for a zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = total_tls::create(
///         "example",
///         TotalTlsArgs::builder()
///             .certificate_authority("lets_encrypt")
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/totalTls:TotalTls example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod total_tls {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TotalTlsArgs {
        /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
        #[builder(into, default)]
        pub certificate_authority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable Total TLS for the zone.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TotalTlsResult {
        /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
        pub certificate_authority: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enable Total TLS for the zone.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TotalTlsArgs,
    ) -> TotalTlsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_authority_binding = args
            .certificate_authority
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/totalTls:TotalTls".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthority".into(),
                    value: certificate_authority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TotalTlsResult {
            certificate_authority: o.get_field("certificateAuthority"),
            enabled: o.get_field("enabled"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
