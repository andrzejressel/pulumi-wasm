pub struct FilterArgs {
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub expression: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FilterResult {
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub expression: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: FilterArgs) -> FilterResult {
    let result = crate::bindings::pulumi::cloudflare::filter::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::filter::Args {
            description: args.description.get_inner(),
            expression: args.expression.get_inner(),
            paused: args.paused.get_inner(),
            ref_: args.ref_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    FilterResult {
        description: crate::into_domain(result.description),
        expression: crate::into_domain(result.expression),
        paused: crate::into_domain(result.paused),
        ref_: crate::into_domain(result.ref_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
