#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SplitTunnelArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    #[builder(into, default)]
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    #[builder(into)]
    pub tunnels: pulumi_wasm_rust::Output<Vec<super::types::SplitTunnelTunnel>>,
}
pub struct SplitTunnelResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// The settings policy for which to configure this split tunnel policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The value of the tunnel attributes.
    pub tunnels: pulumi_wasm_rust::Output<Vec<super::types::SplitTunnelTunnel>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: SplitTunnelArgs) -> SplitTunnelResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let mode_binding = args.mode.get_inner();
    let policy_id_binding = args.policy_id.get_inner();
    let tunnels_binding = args.tunnels.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/splitTunnel:SplitTunnel".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "mode".into(),
                value: &mode_binding,
            },
            register_interface::ObjectField {
                name: "policyId".into(),
                value: &policy_id_binding,
            },
            register_interface::ObjectField {
                name: "tunnels".into(),
                value: &tunnels_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "mode".into() },
            register_interface::ResultField { name : "policyId".into() },
            register_interface::ResultField { name : "tunnels".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    SplitTunnelResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        mode: into_domain(hashmap.remove("mode").unwrap()),
        policy_id: into_domain(hashmap.remove("policyId").unwrap()),
        tunnels: into_domain(hashmap.remove("tunnels").unwrap()),
    }
}
