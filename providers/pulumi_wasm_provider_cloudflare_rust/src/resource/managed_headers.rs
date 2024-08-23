pub struct ManagedHeadersArgs {
    pub managed_request_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    pub managed_response_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ManagedHeadersResult {
    pub managed_request_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    pub managed_response_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {
    let result = crate::bindings::pulumi::cloudflare::managed_headers::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::managed_headers::Args {
            managed_request_headers: args.managed_request_headers.get_inner(),
            managed_response_headers: args.managed_response_headers.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ManagedHeadersResult {
        managed_request_headers: crate::into_domain(result.managed_request_headers),
        managed_response_headers: crate::into_domain(result.managed_response_headers),
        zone_id: crate::into_domain(result.zone_id),
    }
}
