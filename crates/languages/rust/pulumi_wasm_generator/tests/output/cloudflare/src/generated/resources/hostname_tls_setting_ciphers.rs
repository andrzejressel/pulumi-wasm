/// Provides a Cloudflare per-hostname TLS setting resource, specifically for ciphers suites. Used to set ciphers suites for hostnames under the specified zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod hostname_tls_setting_ciphers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostnameTlsSettingCiphersArgs {
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::InputOrOutput<String>,
        /// Ports to use within the IP rule.
        #[builder(into, default)]
        pub ports: pulumi_wasm_rust::InputOrOutput<Option<Vec<i32>>>,
        /// Ciphers suites value.
        #[builder(into)]
        pub values: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HostnameTlsSettingCiphersResult {
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// Ports to use within the IP rule.
        pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        pub updated_at: pulumi_wasm_rust::Output<String>,
        /// Ciphers suites value.
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HostnameTlsSettingCiphersArgs,
    ) -> HostnameTlsSettingCiphersResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let ports_binding = args.ports.get_output(context).get_inner();
        let values_binding = args.values.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ports".into(),
                    value: &ports_binding,
                },
                register_interface::ObjectField {
                    name: "values".into(),
                    value: &values_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostnameTlsSettingCiphersResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            ports: pulumi_wasm_rust::__private::into_domain(o.extract_field("ports")),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updatedAt"),
            ),
            values: pulumi_wasm_rust::__private::into_domain(o.extract_field("values")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
