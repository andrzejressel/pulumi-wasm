#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetTunnelVirtualNetworkResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Comment.
    pub comment: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted virtual networks. If false, exclude deleted virtual networks. If empty, all virtual networks will be included.
    pub is_default: pulumi_wasm_rust::Output<bool>,
    /// The Virtual Network Name.
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetTunnelVirtualNetworkArgs) -> GetTunnelVirtualNetworkResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getTunnelVirtualNetwork:getTunnelVirtualNetwork".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "comment".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "isDefault".into() },
            register_interface::ResultField { name : "name".into() },
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
    GetTunnelVirtualNetworkResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        comment: into_domain(hashmap.remove("comment").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        is_default: into_domain(hashmap.remove("isDefault").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
