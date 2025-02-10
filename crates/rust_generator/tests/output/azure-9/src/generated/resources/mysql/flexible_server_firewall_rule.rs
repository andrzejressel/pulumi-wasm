/// Manages a Firewall Rule for a MySQL Flexible Server.
///
/// ## Example Usage
///
/// ### Single IP Address)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFlexibleServerFirewallRule = flexible_server_firewall_rule::create(
///         "exampleFlexibleServerFirewallRule",
///         FlexibleServerFirewallRuleArgs::builder()
///             .end_ip_address("40.112.8.12")
///             .name("office")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .start_ip_address("40.112.8.12")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### IP Range)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFlexibleServerFirewallRule = flexible_server_firewall_rule::create(
///         "exampleFlexibleServerFirewallRule",
///         FlexibleServerFirewallRuleArgs::builder()
///             .end_ip_address("40.112.255.255")
///             .name("office")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .start_ip_address("40.112.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Allow Access To Azure Services)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFlexibleServerFirewallRule = flexible_server_firewall_rule::create(
///         "exampleFlexibleServerFirewallRule",
///         FlexibleServerFirewallRuleArgs::builder()
///             .end_ip_address("0.0.0.0")
///             .name("office")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .start_ip_address("0.0.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MySQL Firewall Rule's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mysql/flexibleServerFirewallRule:FlexibleServerFirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforMySQL/flexibleServers/flexibleServer1/firewallRules/firewallRule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_server_firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerFirewallRuleArgs {
        /// Specifies the End IP Address associated with this Firewall Rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        #[builder(into)]
        pub end_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the MySQL Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Start IP Address associated with this Firewall Rule.
        #[builder(into)]
        pub start_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerFirewallRuleResult {
        /// Specifies the End IP Address associated with this Firewall Rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        pub end_ip_address: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the MySQL Firewall Rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Start IP Address associated with this Firewall Rule.
        pub start_ip_address: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerFirewallRuleArgs,
    ) -> FlexibleServerFirewallRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let end_ip_address_binding = args.end_ip_address.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_name_binding = args.server_name.get_output(context);
        let start_ip_address_binding = args.start_ip_address.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerFirewallRule:FlexibleServerFirewallRule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endIpAddress".into(),
                    value: end_ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverName".into(),
                    value: server_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startIpAddress".into(),
                    value: start_ip_address_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlexibleServerFirewallRuleResult {
            end_ip_address: o.get_field("endIpAddress"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_name: o.get_field("serverName"),
            start_ip_address: o.get_field("startIpAddress"),
        }
    }
}
