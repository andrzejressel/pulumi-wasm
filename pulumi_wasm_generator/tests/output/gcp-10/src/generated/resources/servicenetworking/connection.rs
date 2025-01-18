/// Manages a private VPC connection with a GCP service provider. For more information see
/// [the official documentation](https://cloud.google.com/vpc/docs/configure-private-services-access#creating-connection)
/// and
/// [API](https://cloud.google.com/service-infrastructure/docs/service-networking/reference/rest/v1/services.connections).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = connection::create(
///         "default",
///         ConnectionArgs::builder()
///             .network("${peeringNetwork.id}")
///             .reserved_peering_ranges(vec!["${privateIpAlloc.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let peeringNetwork = network::create(
///         "peeringNetwork",
///         NetworkArgs::builder().name("peering-network").build_struct(),
///     );
///     let peeringRoutes = network_peering_routes_config::create(
///         "peeringRoutes",
///         NetworkPeeringRoutesConfigArgs::builder()
///             .export_custom_routes(true)
///             .import_custom_routes(true)
///             .network("${peeringNetwork.name}")
///             .peering("${default.peering}")
///             .build_struct(),
///     );
///     let privateIpAlloc = global_address::create(
///         "privateIpAlloc",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("private-ip-alloc")
///             .network("${peeringNetwork.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ServiceNetworkingConnection can be imported using any of these accepted formats
///
/// * `{{peering-network}}:{{service}}`
///
/// * `projects/{{project}}/global/networks/{{peering-network}}:{{service}}`
///
/// When using the `pulumi import` command, NAME_HERE can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/connection:Connection default {{peering-network}}:{{service}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicenetworking/connection:Connection default /projects/{{project}}/global/networks/{{peering-network}}:{{service}}
/// ```
///
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of VPC network connected with service producers using VPC peering.
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// Named IP address range(s) of PEERING type reserved for
        /// this service provider. Note that invoking this method with a different range when connection
        /// is already established will not reallocate already provisioned service producer subnetworks.
        #[builder(into)]
        pub reserved_peering_ranges: pulumi_wasm_rust::Output<Vec<String>>,
        /// Provider peering service that is managing peering connectivity for a
        /// service provider organization. For Google services that support this functionality it is
        /// 'servicenetworking.googleapis.com'.
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
        /// When set to true, enforce an update of the reserved peering ranges on the existing service networking connection in case of a new connection creation failure.
        #[builder(into, default)]
        pub update_on_creation_fail: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of VPC network connected with service producers using VPC peering.
        pub network: pulumi_wasm_rust::Output<String>,
        /// (Computed) The name of the VPC Network Peering connection that was created by the service producer.
        pub peering: pulumi_wasm_rust::Output<String>,
        /// Named IP address range(s) of PEERING type reserved for
        /// this service provider. Note that invoking this method with a different range when connection
        /// is already established will not reallocate already provisioned service producer subnetworks.
        pub reserved_peering_ranges: pulumi_wasm_rust::Output<Vec<String>>,
        /// Provider peering service that is managing peering connectivity for a
        /// service provider organization. For Google services that support this functionality it is
        /// 'servicenetworking.googleapis.com'.
        pub service: pulumi_wasm_rust::Output<String>,
        /// When set to true, enforce an update of the reserved peering ranges on the existing service networking connection in case of a new connection creation failure.
        pub update_on_creation_fail: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let network_binding = args.network.get_inner();
        let reserved_peering_ranges_binding = args.reserved_peering_ranges.get_inner();
        let service_binding = args.service.get_inner();
        let update_on_creation_fail_binding = args.update_on_creation_fail.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:servicenetworking/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "reservedPeeringRanges".into(),
                    value: &reserved_peering_ranges_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "updateOnCreationFail".into(),
                    value: &update_on_creation_fail_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "peering".into(),
                },
                register_interface::ResultField {
                    name: "reservedPeeringRanges".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "updateOnCreationFail".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionResult {
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            peering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peering").unwrap(),
            ),
            reserved_peering_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedPeeringRanges").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            update_on_creation_fail: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateOnCreationFail").unwrap(),
            ),
        }
    }
}
