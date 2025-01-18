/// Allows you to manage an Azure SQL Outbound Firewall Rule.
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
///     let exampleOutboundFirewallRule = outbound_firewall_rule::create(
///         "exampleOutboundFirewallRule",
///         OutboundFirewallRuleArgs::builder()
///             .name("sqlexamplefdqn.database.windows.net")
///             .server_id("${exampleServer.id}")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("mysqlserver")
///             .outbound_network_restriction_enabled(true)
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Outbound Firewall Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/outboundFirewallRule:OutboundFirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver/outboundFirewallRules/fqdn1
/// ```
///
pub mod outbound_firewall_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutboundFirewallRuleArgs {
        /// The name of the outbound firewall rule. This should be a FQDN. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the SQL Server on which to create the Outbound Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OutboundFirewallRuleResult {
        /// The name of the outbound firewall rule. This should be a FQDN. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the SQL Server on which to create the Outbound Firewall Rule. Changing this forces a new resource to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OutboundFirewallRuleArgs,
    ) -> OutboundFirewallRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/outboundFirewallRule:OutboundFirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutboundFirewallRuleResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
        }
    }
}
