/// Provides authorization rules for AWS Client VPN endpoints. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = authorization_rule::create(
///         "example",
///         AuthorizationRuleArgs::builder()
///             .authorize_all_groups(true)
///             .client_vpn_endpoint_id("${exampleAwsEc2ClientVpnEndpoint.id}")
///             .target_network_cidr("${exampleAwsSubnet.cidrBlock}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using the endpoint ID, target network CIDR, and group name:
///
/// __Using `pulumi import` to import__ AWS Client VPN authorization rules using the endpoint ID and target network CIDR. If there is a specific group name, include that also. All values are separated by a `,`. For example:
///
/// Using the endpoint ID and target network CIDR:
///
/// ```sh
/// $ pulumi import aws:ec2clientvpn/authorizationRule:AuthorizationRule example cvpn-endpoint-0ac3a1abbccddd666,10.1.0.0/24
/// ```
/// Using the endpoint ID, target network CIDR, and group name:
///
/// ```sh
/// $ pulumi import aws:ec2clientvpn/authorizationRule:AuthorizationRule example cvpn-endpoint-0ac3a1abbccddd666,10.1.0.0/24,team-a
/// ```
pub mod authorization_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationRuleArgs {
        /// The ID of the group to which the authorization rule grants access. One of `access_group_id` or `authorize_all_groups` must be set.
        #[builder(into, default)]
        pub access_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the authorization rule grants access to all clients. One of `access_group_id` or `authorize_all_groups` must be set.
        #[builder(into, default)]
        pub authorize_all_groups: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Client VPN endpoint.
        #[builder(into)]
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// A brief description of the authorization rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.
        #[builder(into)]
        pub target_network_cidr: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizationRuleResult {
        /// The ID of the group to which the authorization rule grants access. One of `access_group_id` or `authorize_all_groups` must be set.
        pub access_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the authorization rule grants access to all clients. One of `access_group_id` or `authorize_all_groups` must be set.
        pub authorize_all_groups: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Client VPN endpoint.
        pub client_vpn_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// A brief description of the authorization rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.
        pub target_network_cidr: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthorizationRuleArgs) -> AuthorizationRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_group_id_binding = args.access_group_id.get_inner();
        let authorize_all_groups_binding = args.authorize_all_groups.get_inner();
        let client_vpn_endpoint_id_binding = args.client_vpn_endpoint_id.get_inner();
        let description_binding = args.description.get_inner();
        let target_network_cidr_binding = args.target_network_cidr.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/authorizationRule:AuthorizationRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessGroupId".into(),
                    value: &access_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "authorizeAllGroups".into(),
                    value: &authorize_all_groups_binding,
                },
                register_interface::ObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: &client_vpn_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "targetNetworkCidr".into(),
                    value: &target_network_cidr_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessGroupId".into(),
                },
                register_interface::ResultField {
                    name: "authorizeAllGroups".into(),
                },
                register_interface::ResultField {
                    name: "clientVpnEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "targetNetworkCidr".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthorizationRuleResult {
            access_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessGroupId").unwrap(),
            ),
            authorize_all_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizeAllGroups").unwrap(),
            ),
            client_vpn_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVpnEndpointId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            target_network_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetNetworkCidr").unwrap(),
            ),
        }
    }
}