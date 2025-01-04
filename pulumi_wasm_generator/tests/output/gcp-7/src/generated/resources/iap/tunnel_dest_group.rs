/// Tunnel destination groups represent resources that have the same tunnel access restrictions.
///
///
/// To get more information about TunnelDestGroup, see:
///
/// * [API documentation](https://cloud.google.com/iap/docs/reference/rest/v1/projects.iap_tunnel.locations.destGroups)
/// * How-to Guides
///     * [Set up IAP TCP forwarding with an IP address or hostname in a Google Cloud or non-Google Cloud environment](https://cloud.google.com/iap/docs/tcp-by-host)
///
/// ## Example Usage
///
/// ### Iap Destgroup
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let destGroup = tunnel_dest_group::create(
///         "destGroup",
///         TunnelDestGroupArgs::builder()
///             .cidrs(vec!["10.1.0.0/16", "192.168.10.0/24",])
///             .group_name("testgroup_2067")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TunnelDestGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}`
///
/// * `{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}`
///
/// * `{{project}}/{{region}}/{{group_name}}`
///
/// * `{{region}}/destGroups/{{group_name}}`
///
/// * `{{region}}/{{group_name}}`
///
/// * `{{group_name}}`
///
/// When using the `pulumi import` command, TunnelDestGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default projects/{{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{project}}/iap_tunnel/locations/{{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{project}}/{{region}}/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{region}}/destGroups/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{region}}/{{group_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iap/tunnelDestGroup:TunnelDestGroup default {{group_name}}
/// ```
///
pub mod tunnel_dest_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelDestGroupArgs {
        /// List of CIDRs that this group applies to.
        #[builder(into, default)]
        pub cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of FQDNs that this group applies to.
        #[builder(into, default)]
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Unique tunnel destination group name.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TunnelDestGroupResult {
        /// List of CIDRs that this group applies to.
        pub cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of FQDNs that this group applies to.
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Unique tunnel destination group name.
        ///
        ///
        /// - - -
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// Full resource name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of the tunnel group. Must be the same as the network resources in the group.
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TunnelDestGroupArgs) -> TunnelDestGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidrs_binding = args.cidrs.get_inner();
        let fqdns_binding = args.fqdns.get_inner();
        let group_name_binding = args.group_name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iap/tunnelDestGroup:TunnelDestGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrs".into(),
                    value: &cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "fqdns".into(),
                    value: &fqdns_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
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
                    name: "cidrs".into(),
                },
                register_interface::ResultField {
                    name: "fqdns".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TunnelDestGroupResult {
            cidrs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrs").unwrap(),
            ),
            fqdns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdns").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
