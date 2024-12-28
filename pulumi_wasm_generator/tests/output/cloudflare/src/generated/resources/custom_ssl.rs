/// Provides a Cloudflare custom SSL resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:CustomSsl
///     properties:
///       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
///       customSslOptions:
///         certificate: '-----INSERT CERTIFICATE-----'
///         privateKey: '-----INSERT PRIVATE KEY-----'
///         bundleMethod: ubiquitous
///         geoRestrictions: us
///         type: legacy_custom
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/customSsl:CustomSsl example <zone_id>/<certificate_id>
/// ```
///
pub mod custom_ssl {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomSslArgs {
        /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub custom_ssl_options: pulumi_wasm_rust::Output<
            Option<super::types::CustomSslCustomSslOptions>,
        >,
        #[builder(into, default)]
        pub custom_ssl_priorities: pulumi_wasm_rust::Output<
            Option<Vec<super::types::CustomSslCustomSslPriority>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomSslResult {
        /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
        pub custom_ssl_options: pulumi_wasm_rust::Output<
            Option<super::types::CustomSslCustomSslOptions>,
        >,
        pub custom_ssl_priorities: pulumi_wasm_rust::Output<
            Option<Vec<super::types::CustomSslCustomSslPriority>>,
        >,
        pub expires_on: pulumi_wasm_rust::Output<String>,
        pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
        pub issuer: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub signature: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomSslArgs) -> CustomSslResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_ssl_options_binding = args.custom_ssl_options.get_inner();
        let custom_ssl_priorities_binding = args.custom_ssl_priorities.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/customSsl:CustomSsl".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customSslOptions".into(),
                    value: &custom_ssl_options_binding,
                },
                register_interface::ObjectField {
                    name: "customSslPriorities".into(),
                    value: &custom_ssl_priorities_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customSslOptions".into(),
                },
                register_interface::ResultField {
                    name: "customSslPriorities".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "hosts".into(),
                },
                register_interface::ResultField {
                    name: "issuer".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "signature".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "uploadedOn".into(),
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
        CustomSslResult {
            custom_ssl_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSslOptions").unwrap(),
            ),
            custom_ssl_priorities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSslPriorities").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            hosts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hosts").unwrap(),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuer").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            signature: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signature").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            uploaded_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploadedOn").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
