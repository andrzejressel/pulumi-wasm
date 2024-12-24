#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetNetworkArgs {
    /// The name of the Docker network.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetNetworkResult {
    /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// The ID of this resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If `true`, the network is internal.
    pub internal: pulumi_wasm_rust::Output<bool>,
    /// The IPAM configuration options
    pub ipam_configs: pulumi_wasm_rust::Output<
        Vec<super::super::types::GetNetworkIpamConfig>,
    >,
    /// The name of the Docker network.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
    pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Scope of the network. One of `swarm`, `global`, or `local`.
    pub scope: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetNetworkArgs) -> GetNetworkResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "docker:index/getNetwork:getNetwork".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "driver".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "internal".into() },
            register_interface::ResultField { name : "ipamConfigs".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "options".into() },
            register_interface::ResultField { name : "scope".into() },
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
    GetNetworkResult {
        driver: into_domain(hashmap.remove("driver").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        internal: into_domain(hashmap.remove("internal").unwrap()),
        ipam_configs: into_domain(hashmap.remove("ipamConfigs").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        options: into_domain(hashmap.remove("options").unwrap()),
        scope: into_domain(hashmap.remove("scope").unwrap()),
    }
}
