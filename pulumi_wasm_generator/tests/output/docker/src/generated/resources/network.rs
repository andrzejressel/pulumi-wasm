/// <!-- Bug: Type and Name are switched -->
/// `docker.Network` provides a docker network resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let privateNetwork = network::create(
///         "privateNetwork",
///         NetworkArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `network` as follows
///
/// ```shell
/// docker network create foo
/// ````
///
/// prints the long ID
///
/// ```text
/// 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = network::create("foo", NetworkArgs::builder().name("foo").build_struct());
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
pub mod network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// Enable manual container attachment to the network.
        #[builder(into, default)]
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        #[builder(into, default)]
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        #[builder(into, default)]
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        #[builder(into, default)]
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        #[builder(into, default)]
        pub internal: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPAM configuration options
        #[builder(into, default)]
        pub ipam_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::types::NetworkIpamConfig>>,
        >,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        #[builder(into, default)]
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        #[builder(into, default)]
        pub ipam_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        #[builder(into, default)]
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::NetworkLabel>>>,
        /// The name of the Docker network.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// Enable manual container attachment to the network.
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        pub driver: pulumi_wasm_rust::Output<String>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        pub internal: pulumi_wasm_rust::Output<bool>,
        /// The IPAM configuration options
        pub ipam_configs: pulumi_wasm_rust::Output<Vec<super::types::NetworkIpamConfig>>,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        pub ipam_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::NetworkLabel>>>,
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
    pub fn create(name: &str, args: NetworkArgs) -> NetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachable_binding = args.attachable.get_inner();
        let check_duplicate_binding = args.check_duplicate.get_inner();
        let driver_binding = args.driver.get_inner();
        let ingress_binding = args.ingress.get_inner();
        let internal_binding = args.internal.get_inner();
        let ipam_configs_binding = args.ipam_configs.get_inner();
        let ipam_driver_binding = args.ipam_driver.get_inner();
        let ipam_options_binding = args.ipam_options.get_inner();
        let ipv6_binding = args.ipv6.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let options_binding = args.options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachable".into(),
                    value: &attachable_binding,
                },
                register_interface::ObjectField {
                    name: "checkDuplicate".into(),
                    value: &check_duplicate_binding,
                },
                register_interface::ObjectField {
                    name: "driver".into(),
                    value: &driver_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "internal".into(),
                    value: &internal_binding,
                },
                register_interface::ObjectField {
                    name: "ipamConfigs".into(),
                    value: &ipam_configs_binding,
                },
                register_interface::ObjectField {
                    name: "ipamDriver".into(),
                    value: &ipam_driver_binding,
                },
                register_interface::ObjectField {
                    name: "ipamOptions".into(),
                    value: &ipam_options_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachable".into(),
                },
                register_interface::ResultField {
                    name: "checkDuplicate".into(),
                },
                register_interface::ResultField {
                    name: "driver".into(),
                },
                register_interface::ResultField {
                    name: "ingress".into(),
                },
                register_interface::ResultField {
                    name: "internal".into(),
                },
                register_interface::ResultField {
                    name: "ipamConfigs".into(),
                },
                register_interface::ResultField {
                    name: "ipamDriver".into(),
                },
                register_interface::ResultField {
                    name: "ipamOptions".into(),
                },
                register_interface::ResultField {
                    name: "ipv6".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkResult {
            attachable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachable").unwrap(),
            ),
            check_duplicate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkDuplicate").unwrap(),
            ),
            driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driver").unwrap(),
            ),
            ingress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingress").unwrap(),
            ),
            internal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internal").unwrap(),
            ),
            ipam_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamConfigs").unwrap(),
            ),
            ipam_driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamDriver").unwrap(),
            ),
            ipam_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamOptions").unwrap(),
            ),
            ipv6: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
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
