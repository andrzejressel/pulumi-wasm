#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsArgs {
    /// The account identifier to target for the datasource lookups.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    #[builder(into, default)]
    pub filter: pulumi_wasm_rust::Output<
        Option<super::super::types::GetLoadBalancerPoolsFilter>,
    >,
    /// A list of Load Balancer Pools details.
    #[builder(into, default)]
    pub pools: pulumi_wasm_rust::Output<
        Option<Vec<super::super::types::GetLoadBalancerPoolsPool>>,
    >,
}
pub struct GetLoadBalancerPoolsResult {
    /// The account identifier to target for the datasource lookups.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// One or more values used to look up Load Balancer pools. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<
        Option<super::super::types::GetLoadBalancerPoolsFilter>,
    >,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of Load Balancer Pools details.
    pub pools: pulumi_wasm_rust::Output<
        Vec<super::super::types::GetLoadBalancerPoolsPool>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetLoadBalancerPoolsArgs) -> GetLoadBalancerPoolsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let filter_binding = args.filter.get_inner();
    let pools_binding = args.pools.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getLoadBalancerPools:getLoadBalancerPools".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "filter".into(),
                value: &filter_binding,
            },
            register_interface::ObjectField {
                name: "pools".into(),
                value: &pools_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "filter".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "pools".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetLoadBalancerPoolsResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        filter: into_domain(hashmap.remove("filter").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        pools: into_domain(hashmap.remove("pools").unwrap()),
    }
}
