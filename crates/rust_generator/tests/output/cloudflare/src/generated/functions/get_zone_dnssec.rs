#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zone_dnssec {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneDnssecArgs {
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetZoneDnssecResult {
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
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key Tag for the Zone DNSSEC.
        pub key_tag: pulumi_gestalt_rust::Output<i32>,
        /// Key type used for Zone DNSSEC.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// Public Key for the Zone DNSSEC.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// The status of the Zone DNSSEC.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetZoneDnssecArgs,
    ) -> GetZoneDnssecResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getZoneDnssec:getZoneDnssec".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZoneDnssecResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_tag: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyTag"),
            ),
            key_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyType"),
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
