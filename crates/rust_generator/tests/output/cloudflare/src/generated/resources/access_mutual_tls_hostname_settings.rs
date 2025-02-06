/// Provides a Cloudflare Access Mutual TLS Certificate Settings resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_mutual_tls_hostname_settings::create(
///         "example",
///         AccessMutualTlsHostnameSettingsArgs::builder()
///             .settings(
///                 vec![
///                     AccessMutualTlsHostnameSettingsSetting::builder().chinaNetwork(false)
///                     .clientCertificateForwarding(true).hostname("example.com")
///                     .build_struct(),
///                 ],
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Account level mTLS hostname settings import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example account/<account_id>
/// ```
///
/// Zone level mTLS hostname settings import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example zone/<zone_id>
/// ```
///
pub mod access_mutual_tls_hostname_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessMutualTlsHostnameSettingsArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessMutualTlsHostnameSettingsResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessMutualTlsHostnameSettingsArgs,
    ) -> AccessMutualTlsHostnameSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let settings_binding = args.settings.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessMutualTlsHostnameSettingsResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
