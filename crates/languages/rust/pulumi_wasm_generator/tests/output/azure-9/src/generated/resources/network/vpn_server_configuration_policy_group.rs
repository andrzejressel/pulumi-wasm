/// Manages a VPN Server Configuration Policy Group.
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
pub mod vpn_server_configuration_policy_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnServerConfigurationPolicyGroupArgs {
        /// Is this a default VPN Server Configuration Policy Group? Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub is_default: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Name which should be used for this VPN Server Configuration Policy Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `policy` blocks as documented below.
        #[builder(into)]
        pub policies: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::network::VpnServerConfigurationPolicyGroupPolicy>,
        >,
        /// The priority of this VPN Server Configuration Policy Group. Defaults to `0`.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the VPN Server Configuration that the VPN Server Configuration Policy Group belongs to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vpn_server_configuration_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnServerConfigurationPolicyGroupResult {
        /// Is this a default VPN Server Configuration Policy Group? Defaults to `false`. Changing this forces a new resource to be created.
        pub is_default: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Name which should be used for this VPN Server Configuration Policy Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `policy` blocks as documented below.
        pub policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::VpnServerConfigurationPolicyGroupPolicy>,
        >,
        /// The priority of this VPN Server Configuration Policy Group. Defaults to `0`.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the VPN Server Configuration that the VPN Server Configuration Policy Group belongs to. Changing this forces a new resource to be created.
        pub vpn_server_configuration_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpnServerConfigurationPolicyGroupArgs,
    ) -> VpnServerConfigurationPolicyGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let is_default_binding = args.is_default.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policies_binding = args.policies.get_output(context).get_inner();
        let priority_binding = args.priority.get_output(context).get_inner();
        let vpn_server_configuration_id_binding = args
            .vpn_server_configuration_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/vpnServerConfigurationPolicyGroup:VpnServerConfigurationPolicyGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "isDefault".into(),
                    value: &is_default_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policies".into(),
                    value: &policies_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "vpnServerConfigurationId".into(),
                    value: &vpn_server_configuration_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpnServerConfigurationPolicyGroupResult {
            is_default: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isDefault"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policies"),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            vpn_server_configuration_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpnServerConfigurationId"),
            ),
        }
    }
}
