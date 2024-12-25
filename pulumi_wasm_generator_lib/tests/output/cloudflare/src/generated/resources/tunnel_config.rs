#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TunnelConfigArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<super::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    #[builder(into)]
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}
pub struct TunnelConfigResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Configuration block for Tunnel Configuration.
    pub config: pulumi_wasm_rust::Output<super::types::TunnelConfigConfig>,
    /// Identifier of the Tunnel to target for this configuration.
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: TunnelConfigArgs) -> TunnelConfigResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let config_binding = args.config.get_inner();
    let tunnel_id_binding = args.tunnel_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/tunnelConfig:TunnelConfig".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "config".into(),
                value: &config_binding,
            },
            register_interface::ObjectField {
                name: "tunnelId".into(),
                value: &tunnel_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "config".into() },
            register_interface::ResultField { name : "tunnelId".into() },
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
    TunnelConfigResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        config: into_domain(hashmap.remove("config").unwrap()),
        tunnel_id: into_domain(hashmap.remove("tunnelId").unwrap()),
    }
}
