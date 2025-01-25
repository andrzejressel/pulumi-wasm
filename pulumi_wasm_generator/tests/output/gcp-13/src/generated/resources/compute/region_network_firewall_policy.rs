/// The Compute NetworkFirewallPolicy resource
///
///
///
/// ## Example Usage
///
/// ### Region Network Firewall Policy Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = region_network_firewall_policy::create(
///         "policy",
///         RegionNetworkFirewallPolicyArgs::builder()
///             .description("Terraform test")
///             .name("tf-test-policy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionNetworkFirewallPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/firewallPolicies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionNetworkFirewallPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicy:RegionNetworkFirewallPolicy default projects/{{project}}/regions/{{region}}/firewallPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicy:RegionNetworkFirewallPolicy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicy:RegionNetworkFirewallPolicy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicy:RegionNetworkFirewallPolicy default {{name}}
/// ```
///
pub mod region_network_firewall_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionNetworkFirewallPolicyArgs {
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User-provided name of the Network firewall policy. The name should be unique in the project in which the firewall policy is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of this resource.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionNetworkFirewallPolicyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of the resource. This field is used internally during updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// User-provided name of the Network firewall policy. The name should be unique in the project in which the firewall policy is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The region of this resource.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub region_network_firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// Total count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples.
        pub rule_tuple_count: pulumi_wasm_rust::Output<i32>,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionNetworkFirewallPolicyArgs,
    ) -> RegionNetworkFirewallPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionNetworkFirewallPolicy:RegionNetworkFirewallPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
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
                register_interface::ResultField {
                    name: "regionNetworkFirewallPolicyId".into(),
                },
                register_interface::ResultField {
                    name: "ruleTupleCount".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "selfLinkWithId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionNetworkFirewallPolicyResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
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
            region_network_firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionNetworkFirewallPolicyId").unwrap(),
            ),
            rule_tuple_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ruleTupleCount").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            self_link_with_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLinkWithId").unwrap(),
            ),
        }
    }
}
