pub struct ZoneDnssecArgs {
    pub modified_on: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneDnssecResult {
    pub algorithm: pulumi_wasm_rust::Output<String>,
    pub digest: pulumi_wasm_rust::Output<String>,
    pub digest_algorithm: pulumi_wasm_rust::Output<String>,
    pub digest_type: pulumi_wasm_rust::Output<String>,
    pub ds: pulumi_wasm_rust::Output<String>,
    pub flags: pulumi_wasm_rust::Output<i32>,
    pub key_tag: pulumi_wasm_rust::Output<i32>,
    pub key_type: pulumi_wasm_rust::Output<String>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub public_key: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneDnssecArgs) -> ZoneDnssecResult {
    let result = crate::bindings::pulumi::cloudflare::zone_dnssec::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_dnssec::Args {
            modified_on: args.modified_on.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneDnssecResult {
        algorithm: crate::into_domain(result.algorithm),
        digest: crate::into_domain(result.digest),
        digest_algorithm: crate::into_domain(result.digest_algorithm),
        digest_type: crate::into_domain(result.digest_type),
        ds: crate::into_domain(result.ds),
        flags: crate::into_domain(result.flags),
        key_tag: crate::into_domain(result.key_tag),
        key_type: crate::into_domain(result.key_type),
        modified_on: crate::into_domain(result.modified_on),
        public_key: crate::into_domain(result.public_key),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
