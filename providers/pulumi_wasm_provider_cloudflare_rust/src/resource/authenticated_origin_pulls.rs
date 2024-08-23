pub struct AuthenticatedOriginPullsArgs {
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct AuthenticatedOriginPullsResult {
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AuthenticatedOriginPullsArgs) -> AuthenticatedOriginPullsResult {
    let result = crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::Args {
            authenticated_origin_pulls_certificate: args
                .authenticated_origin_pulls_certificate
                .get_inner(),
            enabled: args.enabled.get_inner(),
            hostname: args.hostname.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AuthenticatedOriginPullsResult {
        authenticated_origin_pulls_certificate: crate::into_domain(
            result.authenticated_origin_pulls_certificate,
        ),
        enabled: crate::into_domain(result.enabled),
        hostname: crate::into_domain(result.hostname),
        zone_id: crate::into_domain(result.zone_id),
    }
}
