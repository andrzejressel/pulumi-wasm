/// Represents a Router resource.
///
///
/// To get more information about Router, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routers)
/// * How-to Guides
///     * [Google Cloud Router](https://cloud.google.com/router/docs/)
///
/// ## Example Usage
///
/// ### Router Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = router::create(
///         "foobar",
///         RouterArgs::builder()
///             .bgp(
///                 RouterBgp::builder()
///                     .advertiseMode("CUSTOM")
///                     .advertisedGroups(vec!["ALL_SUBNETS",])
///                     .advertisedIpRanges(
///                         vec![
///                             RouterBgpAdvertisedIpRange::builder().range("1.2.3.4")
///                             .build_struct(), RouterBgpAdvertisedIpRange::builder()
///                             .range("6.7.0.0/16").build_struct(),
///                         ],
///                     )
///                     .asn(64514)
///                     .build_struct(),
///             )
///             .name("my-router")
///             .network("${foobarNetwork.name}")
///             .build_struct(),
///     );
///     let foobarNetwork = network::create(
///         "foobarNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Router Encrypted Interconnect
///
///
/// ```yaml
/// resources:
///   encrypted-interconnect-router:
///     type: gcp:compute:Router
///     properties:
///       name: test-router
///       network: ${network.name}
///       encryptedInterconnectRouter: true
///       bgp:
///         asn: 64514
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// Router can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/routers/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Router can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default projects/{{project}}/regions/{{region}}/routers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/router:Router default {{name}}
/// ```
///
pub mod router {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouterArgs {
        /// BGP information specific to this router.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bgp: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RouterBgp>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Indicates if a router is dedicated for use with encrypted VLAN
        /// attachments (interconnectAttachments).
        #[builder(into, default)]
        pub encrypted_interconnect_router: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the network to which this router belongs.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region where the router resides.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouterResult {
        /// BGP information specific to this router.
        /// Structure is documented below.
        pub bgp: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RouterBgp>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if a router is dedicated for use with encrypted VLAN
        /// attachments (interconnectAttachments).
        pub encrypted_interconnect_router: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A reference to the network to which this router belongs.
        ///
        ///
        /// - - -
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Region where the router resides.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouterArgs,
    ) -> RouterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_binding = args.bgp.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let encrypted_interconnect_router_binding = args
            .encrypted_interconnect_router
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/router:Router".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgp".into(),
                    value: &bgp_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptedInterconnectRouter".into(),
                    value: &encrypted_interconnect_router_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bgp".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptedInterconnectRouter".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouterResult {
            bgp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bgp").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encrypted_interconnect_router: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedInterconnectRouter").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
