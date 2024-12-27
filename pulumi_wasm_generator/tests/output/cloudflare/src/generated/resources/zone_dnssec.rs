/// Provides a Cloudflare resource to create and modify zone DNSSEC settings.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone::create(
///         "example",
///         ZoneArgs::builder().zone("example.com").build_struct(),
///     );
///     let exampleZoneDnssec = zone_dnssec::create(
///         "exampleZoneDnssec",
///         ZoneDnssecArgs::builder().zone_id("${example.id}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneDnssec:ZoneDnssec example <zone_id>
/// ```
///
pub mod zone_dnssec {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneDnssecArgs {
        /// Zone DNSSEC updated time.
        #[builder(into, default)]
        pub modified_on: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneDnssecResult {
        /// Zone DNSSEC algorithm.
        pub algorithm: pulumi_wasm_rust::Output<String>,
        /// Zone DNSSEC digest.
        pub digest: pulumi_wasm_rust::Output<String>,
        /// Digest algorithm use for Zone DNSSEC.
        pub digest_algorithm: pulumi_wasm_rust::Output<String>,
        /// Digest Type for Zone DNSSEC.
        pub digest_type: pulumi_wasm_rust::Output<String>,
        /// DS for the Zone DNSSEC.
        pub ds: pulumi_wasm_rust::Output<String>,
        /// Zone DNSSEC flags.
        pub flags: pulumi_wasm_rust::Output<i32>,
        /// Key Tag for the Zone DNSSEC.
        pub key_tag: pulumi_wasm_rust::Output<i32>,
        /// Key type used for Zone DNSSEC.
        pub key_type: pulumi_wasm_rust::Output<String>,
        /// Zone DNSSEC updated time.
        pub modified_on: pulumi_wasm_rust::Output<String>,
        /// Public Key for the Zone DNSSEC.
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// The status of the Zone DNSSEC.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ZoneDnssecArgs) -> ZoneDnssecResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let modified_on_binding = args.modified_on.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneDnssec:ZoneDnssec".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modifiedOn".into(),
                    value: &modified_on_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "algorithm".into(),
                },
                register_interface::ResultField {
                    name: "digest".into(),
                },
                register_interface::ResultField {
                    name: "digestAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "digestType".into(),
                },
                register_interface::ResultField {
                    name: "ds".into(),
                },
                register_interface::ResultField {
                    name: "flags".into(),
                },
                register_interface::ResultField {
                    name: "keyTag".into(),
                },
                register_interface::ResultField {
                    name: "keyType".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        ZoneDnssecResult {
            algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("algorithm").unwrap(),
            ),
            digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digest").unwrap(),
            ),
            digest_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digestAlgorithm").unwrap(),
            ),
            digest_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digestType").unwrap(),
            ),
            ds: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ds").unwrap()),
            flags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flags").unwrap(),
            ),
            key_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyTag").unwrap(),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyType").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
