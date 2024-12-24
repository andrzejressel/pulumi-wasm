#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub is_deleted: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// ID of the tunnel.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted tunnels. If false, exclude deleted tunnels. If empty, all tunnels will be included. **Modifying this attribute will force creation of a new resource.**
    pub is_deleted: pulumi_wasm_rust::Output<Option<bool>>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// Whether the tunnel can be configured remotely from the Zero Trust dashboard.
    pub remote_config: pulumi_wasm_rust::Output<bool>,
    /// The status of the tunnel. Available values: `inactive`, `degraded`, `healthy`, `down`.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The type of the tunnel. Available values: `cfd_tunnel`, `warp_connector`.
    pub tunnel_type: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetTunnelArgs) -> GetTunnelResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let is_deleted_binding = args.is_deleted.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getTunnel:getTunnel".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "isDeleted".into(),
                value: &is_deleted_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "isDeleted".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "remoteConfig".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "tunnelType".into() },
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
    GetTunnelResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        is_deleted: into_domain(hashmap.remove("isDeleted").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        remote_config: into_domain(hashmap.remove("remoteConfig").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        tunnel_type: into_domain(hashmap.remove("tunnelType").unwrap()),
    }
}
