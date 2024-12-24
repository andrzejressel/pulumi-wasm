#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DeviceManagedNetworksArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    #[builder(into)]
    pub config: pulumi_wasm_rust::Output<super::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}
pub struct DeviceManagedNetworksResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration containing information for the WARP client to detect the managed network.
    pub config: pulumi_wasm_rust::Output<super::types::DeviceManagedNetworksConfig>,
    /// The name of the Device Managed Network. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The type of Device Managed Network. Available values: `tls`.
    pub type_: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: DeviceManagedNetworksArgs,
) -> DeviceManagedNetworksResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let config_binding = args.config.get_inner();
    let name_binding = args.name.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks".into(),
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
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "config".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "type".into() },
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
    DeviceManagedNetworksResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        config: into_domain(hashmap.remove("config").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
