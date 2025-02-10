/// Provides a Cloudflare per-hostname TLS setting resource, specifically for ciphers suites. Used to set ciphers suites for hostnames under the specified zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hostname_tls_setting_ciphers::create(
///         "example",
///         HostnameTlsSettingCiphersArgs::builder()
///             .hostname("sub.example.com")
///             .values(vec!["ECDHE-RSA-AES128-GCM-SHA256",])
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers example <zone_id>/<hostname>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hostname_tls_setting_ciphers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostnameTlsSettingCiphersArgs {
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Ports to use within the IP rule.
        #[builder(into, default)]
        pub ports: pulumi_gestalt_rust::InputOrOutput<Option<Vec<i32>>>,
        /// Ciphers suites value.
        #[builder(into)]
        pub values: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HostnameTlsSettingCiphersResult {
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// Ports to use within the IP rule.
        pub ports: pulumi_gestalt_rust::Output<Option<Vec<i32>>>,
        pub updated_at: pulumi_gestalt_rust::Output<String>,
        /// Ciphers suites value.
        pub values: pulumi_gestalt_rust::Output<Vec<String>>,
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
        args: HostnameTlsSettingCiphersArgs,
    ) -> HostnameTlsSettingCiphersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hostname_binding = args.hostname.get_output(context);
        let ports_binding = args.ports.get_output(context);
        let values_binding = args.values.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ports".into(),
                    value: ports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "values".into(),
                    value: values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostnameTlsSettingCiphersResult {
            created_at: o.get_field("createdAt"),
            hostname: o.get_field("hostname"),
            ports: o.get_field("ports"),
            updated_at: o.get_field("updatedAt"),
            values: o.get_field("values"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
