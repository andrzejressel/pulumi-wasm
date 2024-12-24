#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustTunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    #[builder(into, default)]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    #[builder(into, default)]
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct ZeroTrustTunnelVirtualNetworkResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZeroTrustTunnelVirtualNetworkArgs,
) -> ZeroTrustTunnelVirtualNetworkResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let comment_binding = args.comment.get_inner();
    let is_default_network_binding = args.is_default_network.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustTunnelVirtualNetwork:ZeroTrustTunnelVirtualNetwork"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "comment".into(),
                value: &comment_binding,
            },
            register_interface::ObjectField {
                name: "isDefaultNetwork".into(),
                value: &is_default_network_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "comment".into() },
            register_interface::ResultField { name : "isDefaultNetwork".into() },
            register_interface::ResultField { name : "name".into() },
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
    ZeroTrustTunnelVirtualNetworkResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        comment: into_domain(hashmap.remove("comment").unwrap()),
        is_default_network: into_domain(hashmap.remove("isDefaultNetwork").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
