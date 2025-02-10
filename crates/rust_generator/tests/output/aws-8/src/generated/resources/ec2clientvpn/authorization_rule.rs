/// Provides authorization rules for AWS Client VPN endpoints. For more information on usage, please see the
/// [AWS Client VPN Administrator's Guide](https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/what-is.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorization_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizationRuleArgs {
        /// The ID of the group to which the authorization rule grants access. One of `access_group_id` or `authorize_all_groups` must be set.
        #[builder(into, default)]
        pub access_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the authorization rule grants access to all clients. One of `access_group_id` or `authorize_all_groups` must be set.
        #[builder(into, default)]
        pub authorize_all_groups: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Client VPN endpoint.
        #[builder(into)]
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A brief description of the authorization rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.
        #[builder(into)]
        pub target_network_cidr: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AuthorizationRuleResult {
        /// The ID of the group to which the authorization rule grants access. One of `access_group_id` or `authorize_all_groups` must be set.
        pub access_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether the authorization rule grants access to all clients. One of `access_group_id` or `authorize_all_groups` must be set.
        pub authorize_all_groups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Client VPN endpoint.
        pub client_vpn_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// A brief description of the authorization rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR notation, of the network to which the authorization rule applies.
        pub target_network_cidr: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizationRuleArgs,
    ) -> AuthorizationRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_group_id_binding = args.access_group_id.get_output(context);
        let authorize_all_groups_binding = args.authorize_all_groups.get_output(context);
        let client_vpn_endpoint_id_binding = args
            .client_vpn_endpoint_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let target_network_cidr_binding = args.target_network_cidr.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2clientvpn/authorizationRule:AuthorizationRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessGroupId".into(),
                    value: access_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizeAllGroups".into(),
                    value: authorize_all_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientVpnEndpointId".into(),
                    value: client_vpn_endpoint_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetNetworkCidr".into(),
                    value: target_network_cidr_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizationRuleResult {
            access_group_id: o.get_field("accessGroupId"),
            authorize_all_groups: o.get_field("authorizeAllGroups"),
            client_vpn_endpoint_id: o.get_field("clientVpnEndpointId"),
            description: o.get_field("description"),
            target_network_cidr: o.get_field("targetNetworkCidr"),
        }
    }
}
