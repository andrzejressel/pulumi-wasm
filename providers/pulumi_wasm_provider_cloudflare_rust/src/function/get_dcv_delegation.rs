//! Use this data source to retrieve the DCV Delegation unique identifier for a zone.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetDcvDelegationArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetDcvDelegationResult {
    /// The DCV Delegation hostname
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The DCV Delegation unique identifier
    pub id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetDcvDelegationArgs
) -> GetDcvDelegationResult {

    let result = crate::bindings::pulumi::cloudflare::get_dcv_delegation::invoke(
        &crate::bindings::pulumi::cloudflare::get_dcv_delegation::Args {
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetDcvDelegationResult {
        hostname: crate::into_domain(result.hostname),
        id: crate::into_domain(result.id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
