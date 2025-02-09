/// Allows you to manage an Azure SQL Firewall Rule.
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
///     let exampleFirewallRule = firewall_rule::create(
///         "exampleFirewallRule",
///         FirewallRuleArgs::builder()
///             .end_ip_address("10.0.17.62")
///             .name("FirewallRule1")
///             .server_id("${exampleServer.id}")
///             .start_ip_address("10.0.17.62")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("4dm1n157r470r")
///             .administrator_login_password("4-v3ry-53cr37-p455w0rd")
///             .location("${example.location}")
///             .name("mysqlserver")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SQL Firewall Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/firewallRule:FirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/servers/myserver/firewallRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// The ending IP address to allow through the firewall for this rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        #[builder(into)]
        pub end_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the firewall rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the SQL Server on which to create the Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The starting IP address to allow through the firewall for this rule.
        #[builder(into)]
        pub start_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// The ending IP address to allow through the firewall for this rule.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        pub end_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the firewall rule. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the SQL Server on which to create the Firewall Rule. Changing this forces a new resource to be created.
        pub server_id: pulumi_gestalt_rust::Output<String>,
        /// The starting IP address to allow through the firewall for this rule.
        pub start_ip_address: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let end_ip_address_binding = args.end_ip_address.get_output(context);
        let name_binding = args.name.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let start_ip_address_binding = args.start_ip_address.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/firewallRule:FirewallRule".into(),
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
                    name: "serverId".into(),
                    value: server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startIpAddress".into(),
                    value: start_ip_address_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallRuleResult {
            end_ip_address: o.get_field("endIpAddress"),
            name: o.get_field("name"),
            server_id: o.get_field("serverId"),
            start_ip_address: o.get_field("startIpAddress"),
        }
    }
}
