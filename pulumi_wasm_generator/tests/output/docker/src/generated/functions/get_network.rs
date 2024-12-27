pub mod get_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkArgs {
        /// The name of the Docker network.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkArgs) -> GetNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "driver".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "internal".into(),
                },
                register_interface::ResultField {
                    name: "ipamConfigs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkResult {
            driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driver").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            internal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internal").unwrap(),
            ),
            ipam_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamConfigs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
