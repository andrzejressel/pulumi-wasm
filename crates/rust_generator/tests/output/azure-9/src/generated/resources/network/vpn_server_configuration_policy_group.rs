/// Manages a VPN Server Configuration Policy Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleVpnServerConfiguration = vpn_server_configuration::create(
///         "exampleVpnServerConfiguration",
///         VpnServerConfigurationArgs::builder()
///             .location("${example.location}")
///             .name("example-VPNSC")
///             .radius(
///                 VpnServerConfigurationRadius::builder()
///                     .servers(
///                         vec![
///                             VpnServerConfigurationRadiusServer::builder()
///                             .address("10.105.1.1").score(15)
///                             .secret("vindicators-the-return-of-worldender")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .vpn_authentication_types(vec!["Radius",])
///             .build_struct(),
///     );
///     let exampleVpnServerConfigurationPolicyGroup = vpn_server_configuration_policy_group::create(
///         "exampleVpnServerConfigurationPolicyGroup",
///         VpnServerConfigurationPolicyGroupArgs::builder()
///             .name("example-VPNSCPG")
///             .policies(
///                 vec![
///                     VpnServerConfigurationPolicyGroupPolicy::builder().name("policy1").
///                     type ("RadiusAzureGroupId").value("6ad1bd08").build_struct(),
///                 ],
///             )
///             .vpn_server_configuration_id("${exampleVpnServerConfiguration.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Server Configuration Policy Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vpnServerConfigurationPolicyGroup:VpnServerConfigurationPolicyGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Network/vpnServerConfigurations/serverConfiguration1/configurationPolicyGroups/configurationPolicyGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_server_configuration_policy_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnServerConfigurationPolicyGroupArgs {
        /// Is this a default VPN Server Configuration Policy Group? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub is_default: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Name which should be used for this VPN Server Configuration Policy Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `policy` blocks as documented below.
        #[builder(into)]
        pub policies: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::network::VpnServerConfigurationPolicyGroupPolicy>,
        >,
        /// The priority of this VPN Server Configuration Policy Group. Defaults to `0`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the VPN Server Configuration that the VPN Server Configuration Policy Group belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vpn_server_configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnServerConfigurationPolicyGroupResult {
        /// Is this a default VPN Server Configuration Policy Group? Defaults to `false`. Changing this forces a new resource to be created.
        pub is_default: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Name which should be used for this VPN Server Configuration Policy Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `policy` blocks as documented below.
        pub policies: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::VpnServerConfigurationPolicyGroupPolicy>,
        >,
        /// The priority of this VPN Server Configuration Policy Group. Defaults to `0`.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the VPN Server Configuration that the VPN Server Configuration Policy Group belongs to. Changing this forces a new resource to be created.
        pub vpn_server_configuration_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnServerConfigurationPolicyGroupArgs,
    ) -> VpnServerConfigurationPolicyGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let is_default_binding = args.is_default.get_output(context);
        let name_binding = args.name.get_output(context);
        let policies_binding = args.policies.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let vpn_server_configuration_id_binding = args
            .vpn_server_configuration_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/vpnServerConfigurationPolicyGroup:VpnServerConfigurationPolicyGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefault".into(),
                    value: is_default_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: policies_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnServerConfigurationId".into(),
                    value: vpn_server_configuration_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnServerConfigurationPolicyGroupResult {
            is_default: o.get_field("isDefault"),
            name: o.get_field("name"),
            policies: o.get_field("policies"),
            priority: o.get_field("priority"),
            vpn_server_configuration_id: o.get_field("vpnServerConfigurationId"),
        }
    }
}
