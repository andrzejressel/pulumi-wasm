/// Manages a Express Route Port.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West US")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("port1")
///             .peering_location("Airtel-Chennai-CLS")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Express Route Ports can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRoutePort:ExpressRoutePort example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/expressRoutePorts/port1
/// ```
///
pub mod express_route_port {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRoutePortArgs {
        /// Bandwidth of the Express Route Port in Gbps. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub bandwidth_in_gbps: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The billing type of the Express Route Port. Possible values are `MeteredData` and `UnlimitedData`. Defaults to `MeteredData`.
        #[builder(into, default)]
        pub billing_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The encapsulation method used for the Express Route Port. Changing this forces a new Express Route Port to be created. Possible values are: `Dot1Q`, `QinQ`.
        #[builder(into)]
        pub encapsulation: pulumi_wasm_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortIdentity>,
        >,
        /// A list of `link` blocks as defined below.
        #[builder(into, default)]
        pub link1: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortLink1>,
        >,
        /// A list of `link` blocks as defined below.
        #[builder(into, default)]
        pub link2: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortLink2>,
        >,
        /// The Azure Region where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Express Route Port. Changing this forces a new Express Route Port to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the peering location that this Express Route Port is physically mapped to. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub peering_location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Express Route Port.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExpressRoutePortResult {
        /// Bandwidth of the Express Route Port in Gbps. Changing this forces a new Express Route Port to be created.
        pub bandwidth_in_gbps: pulumi_wasm_rust::Output<i32>,
        /// The billing type of the Express Route Port. Possible values are `MeteredData` and `UnlimitedData`. Defaults to `MeteredData`.
        pub billing_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The encapsulation method used for the Express Route Port. Changing this forces a new Express Route Port to be created. Possible values are: `Dot1Q`, `QinQ`.
        pub encapsulation: pulumi_wasm_rust::Output<String>,
        /// The EtherType of the Express Route Port.
        pub ethertype: pulumi_wasm_rust::Output<String>,
        /// The resource GUID of the Express Route Port.
        pub guid: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::network::ExpressRoutePortIdentity>,
        >,
        /// A list of `link` blocks as defined below.
        pub link1: pulumi_wasm_rust::Output<
            super::super::types::network::ExpressRoutePortLink1,
        >,
        /// A list of `link` blocks as defined below.
        pub link2: pulumi_wasm_rust::Output<
            super::super::types::network::ExpressRoutePortLink2,
        >,
        /// The Azure Region where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum transmission unit of the Express Route Port.
        pub mtu: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Express Route Port. Changing this forces a new Express Route Port to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the peering location that this Express Route Port is physically mapped to. Changing this forces a new Express Route Port to be created.
        pub peering_location: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Express Route Port.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ExpressRoutePortArgs,
    ) -> ExpressRoutePortResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bandwidth_in_gbps_binding = args
            .bandwidth_in_gbps
            .get_output(context)
            .get_inner();
        let billing_type_binding = args.billing_type.get_output(context).get_inner();
        let encapsulation_binding = args.encapsulation.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let link1_binding = args.link1.get_output(context).get_inner();
        let link2_binding = args.link2.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let peering_location_binding = args
            .peering_location
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRoutePort:ExpressRoutePort".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bandwidthInGbps".into(),
                    value: &bandwidth_in_gbps_binding,
                },
                register_interface::ObjectField {
                    name: "billingType".into(),
                    value: &billing_type_binding,
                },
                register_interface::ObjectField {
                    name: "encapsulation".into(),
                    value: &encapsulation_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "link1".into(),
                    value: &link1_binding,
                },
                register_interface::ObjectField {
                    name: "link2".into(),
                    value: &link2_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "peeringLocation".into(),
                    value: &peering_location_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bandwidthInGbps".into(),
                },
                register_interface::ResultField {
                    name: "billingType".into(),
                },
                register_interface::ResultField {
                    name: "encapsulation".into(),
                },
                register_interface::ResultField {
                    name: "ethertype".into(),
                },
                register_interface::ResultField {
                    name: "guid".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "link1".into(),
                },
                register_interface::ResultField {
                    name: "link2".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mtu".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "peeringLocation".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExpressRoutePortResult {
            bandwidth_in_gbps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bandwidthInGbps").unwrap(),
            ),
            billing_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingType").unwrap(),
            ),
            encapsulation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encapsulation").unwrap(),
            ),
            ethertype: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ethertype").unwrap(),
            ),
            guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guid").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            link1: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("link1").unwrap(),
            ),
            link2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("link2").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mtu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mtu").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            peering_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peeringLocation").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
