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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneDnssecArgs,
    ) -> ZoneDnssecResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let modified_on_binding = args.modified_on.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zoneDnssec:ZoneDnssec".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modifiedOn".into(),
                    value: &modified_on_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneDnssecResult {
            algorithm: o.get_field("algorithm"),
            digest: o.get_field("digest"),
            digest_algorithm: o.get_field("digestAlgorithm"),
            digest_type: o.get_field("digestType"),
            ds: o.get_field("ds"),
            flags: o.get_field("flags"),
            key_tag: o.get_field("keyTag"),
            key_type: o.get_field("keyType"),
            modified_on: o.get_field("modifiedOn"),
            public_key: o.get_field("publicKey"),
            status: o.get_field("status"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
