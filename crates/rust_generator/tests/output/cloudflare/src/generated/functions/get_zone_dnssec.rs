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
        context: &pulumi_gestalt_rust::Context,
        args: GetZoneDnssecArgs,
    ) -> GetZoneDnssecResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZoneDnssec:getZoneDnssec".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZoneDnssecResult {
            algorithm: o.get_field("algorithm"),
            digest: o.get_field("digest"),
            digest_algorithm: o.get_field("digestAlgorithm"),
            digest_type: o.get_field("digestType"),
            ds: o.get_field("ds"),
            flags: o.get_field("flags"),
            id: o.get_field("id"),
            key_tag: o.get_field("keyTag"),
            key_type: o.get_field("keyType"),
            public_key: o.get_field("publicKey"),
            status: o.get_field("status"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
