pub struct CustomHostnameArgs {
    pub custom_metadata:
        pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
    pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
    pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomHostnameResult {
    pub custom_metadata:
        pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
    pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub ownership_verification: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub ownership_verification_http:
        pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: CustomHostnameArgs) -> CustomHostnameResult {
    let result = crate::bindings::pulumi::cloudflare::custom_hostname::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_hostname::Args {
            custom_metadata: args.custom_metadata.get_inner(),
            custom_origin_server: args.custom_origin_server.get_inner(),
            custom_origin_sni: args.custom_origin_sni.get_inner(),
            hostname: args.hostname.get_inner(),
            ssls: args.ssls.get_inner(),
            wait_for_ssl_pending_validation: args.wait_for_ssl_pending_validation.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    CustomHostnameResult {
        custom_metadata: crate::into_domain(result.custom_metadata),
        custom_origin_server: crate::into_domain(result.custom_origin_server),
        custom_origin_sni: crate::into_domain(result.custom_origin_sni),
        hostname: crate::into_domain(result.hostname),
        ownership_verification: crate::into_domain(result.ownership_verification),
        ownership_verification_http: crate::into_domain(result.ownership_verification_http),
        ssls: crate::into_domain(result.ssls),
        status: crate::into_domain(result.status),
        wait_for_ssl_pending_validation: crate::into_domain(result.wait_for_ssl_pending_validation),
        zone_id: crate::into_domain(result.zone_id),
    }
}
