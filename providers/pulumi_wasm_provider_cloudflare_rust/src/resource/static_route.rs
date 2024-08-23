pub struct StaticRouteArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub nexthop: pulumi_wasm_rust::Output<String>,
    pub prefix: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<i32>,
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct StaticRouteResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub nexthop: pulumi_wasm_rust::Output<String>,
    pub prefix: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<i32>,
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

pub fn create(name: &str, args: StaticRouteArgs) -> StaticRouteResult {
    let result = crate::bindings::pulumi::cloudflare::static_route::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::static_route::Args {
            account_id: args.account_id.get_inner(),
            colo_names: args.colo_names.get_inner(),
            colo_regions: args.colo_regions.get_inner(),
            description: args.description.get_inner(),
            nexthop: args.nexthop.get_inner(),
            prefix: args.prefix.get_inner(),
            priority: args.priority.get_inner(),
            weight: args.weight.get_inner(),
        },
    );

    StaticRouteResult {
        account_id: crate::into_domain(result.account_id),
        colo_names: crate::into_domain(result.colo_names),
        colo_regions: crate::into_domain(result.colo_regions),
        description: crate::into_domain(result.description),
        nexthop: crate::into_domain(result.nexthop),
        prefix: crate::into_domain(result.prefix),
        priority: crate::into_domain(result.priority),
        weight: crate::into_domain(result.weight),
    }
}
