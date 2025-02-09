/// Provides a Cloudflare resource to create and modify zone DNSSEC settings.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone_dnssec {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneDnssecArgs {
        /// Zone DNSSEC updated time.
        #[builder(into, default)]
        pub modified_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneDnssecResult {
        /// Zone DNSSEC algorithm.
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        /// Zone DNSSEC digest.
        pub digest: pulumi_gestalt_rust::Output<String>,
        /// Digest algorithm use for Zone DNSSEC.
        pub digest_algorithm: pulumi_gestalt_rust::Output<String>,
        /// Digest Type for Zone DNSSEC.
        pub digest_type: pulumi_gestalt_rust::Output<String>,
        /// DS for the Zone DNSSEC.
        pub ds: pulumi_gestalt_rust::Output<String>,
        /// Zone DNSSEC flags.
        pub flags: pulumi_gestalt_rust::Output<i32>,
        /// Key Tag for the Zone DNSSEC.
        pub key_tag: pulumi_gestalt_rust::Output<i32>,
        /// Key type used for Zone DNSSEC.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// Zone DNSSEC updated time.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// Public Key for the Zone DNSSEC.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// The status of the Zone DNSSEC.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZoneDnssecArgs,
    ) -> ZoneDnssecResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let modified_on_binding_1 = args.modified_on.get_output(context);
        let modified_on_binding = modified_on_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zoneDnssec:ZoneDnssec".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZoneDnssecResult {
            algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("algorithm"),
            ),
            digest: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("digest"),
            ),
            digest_algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("digestAlgorithm"),
            ),
            digest_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("digestType"),
            ),
            ds: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ds")),
            flags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("flags")),
            key_tag: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyTag"),
            ),
            key_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            modified_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modifiedOn"),
            ),
            public_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKey"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
