/// The Compute NetworkFirewallPolicyAssociation resource
///
///
/// To get more information about RegionNetworkFirewallPolicyAssociation, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionNetworkFirewallPolicies/addAssociation)
///
/// ## Example Usage
///
/// ### Region Network Firewall Policy Association
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_network_firewall_policy_association::create(
///         "default",
///         RegionNetworkFirewallPolicyAssociationArgs::builder()
///             .attachment_target("${network.id}")
///             .firewall_policy("${policy.id}")
///             .name("my-association")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let policy = region_network_firewall_policy::create(
///         "policy",
///         RegionNetworkFirewallPolicyArgs::builder()
///             .description("Sample global network firewall policy")
///             .name("my-policy")
///             .project("my-project-name")
///             .region("us-west1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionNetworkFirewallPolicyAssociation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/firewallPolicies/{{firewall_policy}}/associations/{{name}}`
///
/// * `{{project}}/{{region}}/{{firewall_policy}}/{{name}}`
///
/// * `{{region}}/{{firewall_policy}}/{{name}}`
///
/// * `{{project}}/{{firewall_policy}}/{{name}}`
///
/// * `{{firewall_policy}}/{{name}}`
///
/// When using the `pulumi import` command, RegionNetworkFirewallPolicyAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation default projects/{{project}}/regions/{{region}}/firewallPolicies/{{firewall_policy}}/associations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation default {{project}}/{{region}}/{{firewall_policy}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation default {{region}}/{{firewall_policy}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation default {{project}}/{{firewall_policy}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation default {{firewall_policy}}/{{name}}
/// ```
///
pub mod region_network_firewall_policy_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionNetworkFirewallPolicyAssociationArgs {
        /// The target that the firewall policy is attached to.
        #[builder(into)]
        pub attachment_target: pulumi_wasm_rust::Output<String>,
        /// The firewall policy of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub firewall_policy: pulumi_wasm_rust::Output<String>,
        /// The name for an association.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of this resource.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionNetworkFirewallPolicyAssociationResult {
        /// The target that the firewall policy is attached to.
        pub attachment_target: pulumi_wasm_rust::Output<String>,
        /// The firewall policy of the resource.
        ///
        ///
        /// - - -
        pub firewall_policy: pulumi_wasm_rust::Output<String>,
        /// The name for an association.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The location of this resource.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The short name of the firewall policy of the association.
        pub short_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RegionNetworkFirewallPolicyAssociationArgs,
    ) -> RegionNetworkFirewallPolicyAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachment_target_binding = args.attachment_target.get_inner();
        let firewall_policy_binding = args.firewall_policy.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionNetworkFirewallPolicyAssociation:RegionNetworkFirewallPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachmentTarget".into(),
                    value: &attachment_target_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicy".into(),
                    value: &firewall_policy_binding,
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
                    name: "attachmentTarget".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicy".into(),
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
                    name: "shortName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionNetworkFirewallPolicyAssociationResult {
            attachment_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentTarget").unwrap(),
            ),
            firewall_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicy").unwrap(),
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
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
        }
    }
}
