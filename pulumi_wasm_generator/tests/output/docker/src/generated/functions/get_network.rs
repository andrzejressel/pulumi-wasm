pub mod get_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkArgs {
        /// The name of the Docker network.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkArgs,
    ) -> GetNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "docker:index/getNetwork:getNetwork".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkResult {
            driver: pulumi_wasm_rust::__private::into_domain(o.extract_field("driver")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            internal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("internal"),
            ),
            ipam_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipamConfigs"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("options"),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
