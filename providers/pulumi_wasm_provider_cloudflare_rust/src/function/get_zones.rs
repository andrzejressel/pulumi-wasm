//! Use this data source to look up Zone results for use in other resources.
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZonesArgs {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    #[builder(into)]
    pub filter: pulumi_wasm_rust::Output<crate::types::GetZonesFilter>,
}

pub struct GetZonesResult {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<crate::types::GetZonesFilter>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of zone objects.
    pub zones: pulumi_wasm_rust::Output<Vec<crate::types::GetZonesZone>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZonesArgs
) -> GetZonesResult {

    let result = crate::bindings::pulumi::cloudflare::get_zones::invoke(
        &crate::bindings::pulumi::cloudflare::get_zones::Args {
                filter: &args.filter.get_inner(),
        }
    );

    GetZonesResult {
        filter: crate::into_domain(result.filter),
        id: crate::into_domain(result.id),
        zones: crate::into_domain(result.zones),
    }
}
