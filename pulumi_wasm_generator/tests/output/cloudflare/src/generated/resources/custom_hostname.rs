/// Provides a Cloudflare custom hostname (also known as SSL for SaaS) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_hostname::create(
///         "example",
///         CustomHostnameArgs::builder()
///             .hostname("hostname.example.com")
///             .ssls(vec![CustomHostnameSsl::builder().method("txt").build_struct(),])
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/customHostname:CustomHostname example 1d5fdc9e88c8a8c4518b068cd94331fe/0d89c70d-ad9f-4843-b99f-6cc0252067e9
/// ```
///
pub mod custom_hostname {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomHostnameArgs {
        /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
        #[builder(into, default)]
        pub custom_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The custom origin server used for certificates.
        #[builder(into, default)]
        pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
        /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
        #[builder(into, default)]
        pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// SSL properties used when creating the custom hostname.
        #[builder(into, default)]
        pub ssls: pulumi_wasm_rust::Output<Option<Vec<super::types::CustomHostnameSsl>>>,
        /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
        #[builder(into, default)]
        pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomHostnameResult {
        /// Custom metadata associated with custom hostname. Only supports primitive string values, all other values are accessible via the API directly.
        pub custom_metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The custom origin server used for certificates.
        pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
        /// The [custom origin SNI](https://developers.cloudflare.com/ssl/ssl-for-saas/hostname-specific-behavior/custom-origin) used for certificates.
        pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname you intend to request a certificate for. **Modifying this attribute will force creation of a new resource.**
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub ownership_verification: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub ownership_verification_http: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// SSL properties used when creating the custom hostname.
        pub ssls: pulumi_wasm_rust::Output<Option<Vec<super::types::CustomHostnameSsl>>>,
        /// Status of the certificate.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Whether to wait for a custom hostname SSL sub-object to reach status `pending_validation` during creation. Defaults to `false`.
        pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomHostnameArgs) -> CustomHostnameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_metadata_binding = args.custom_metadata.get_inner();
        let custom_origin_server_binding = args.custom_origin_server.get_inner();
        let custom_origin_sni_binding = args.custom_origin_sni.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let ssls_binding = args.ssls.get_inner();
        let wait_for_ssl_pending_validation_binding = args
            .wait_for_ssl_pending_validation
            .get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/customHostname:CustomHostname".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customMetadata".into(),
                    value: &custom_metadata_binding,
                },
                register_interface::ObjectField {
                    name: "customOriginServer".into(),
                    value: &custom_origin_server_binding,
                },
                register_interface::ObjectField {
                    name: "customOriginSni".into(),
                    value: &custom_origin_sni_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ssls".into(),
                    value: &ssls_binding,
                },
                register_interface::ObjectField {
                    name: "waitForSslPendingValidation".into(),
                    value: &wait_for_ssl_pending_validation_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customMetadata".into(),
                },
                register_interface::ResultField {
                    name: "customOriginServer".into(),
                },
                register_interface::ResultField {
                    name: "customOriginSni".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "ownershipVerification".into(),
                },
                register_interface::ResultField {
                    name: "ownershipVerificationHttp".into(),
                },
                register_interface::ResultField {
                    name: "ssls".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "waitForSslPendingValidation".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomHostnameResult {
            custom_metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customMetadata").unwrap(),
            ),
            custom_origin_server: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customOriginServer").unwrap(),
            ),
            custom_origin_sni: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customOriginSni").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            ownership_verification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownershipVerification").unwrap(),
            ),
            ownership_verification_http: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownershipVerificationHttp").unwrap(),
            ),
            ssls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ssls").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            wait_for_ssl_pending_validation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitForSslPendingValidation").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
